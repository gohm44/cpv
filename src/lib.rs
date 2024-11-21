use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Read, Write, BufReader, BufWriter};
use thiserror::Error;
use humansize::{format_size, BINARY};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use walkdir::{WalkDir, Error as WalkdirError};

const BUFFER_SIZE: usize = 8192;

#[derive(Error, Debug)]
pub enum CopyError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("Walk error: {0}")]
    Walk(#[from] WalkdirError),
    #[error("'{0}' is a directory (not copied)")]
    IsADirectory(PathBuf),
    #[error("'{0}' is not a directory")]
    NotADirectory(PathBuf),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct CopyOptions {
    pub preserve_attrs: bool,
    pub force: bool,
    pub verbose: bool,
    pub recursive: bool,
}

#[derive(Debug, Default)]
pub struct CopyStats {
    pub bytes_copied: u64,
    pub files_copied: usize,
    pub dirs_created: usize,
    pub time_taken: std::time::Duration,
}

impl CopyStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn format_summary(&self) -> String {
        format!(
            "Copied {} in {} files ({:.2} MB/s)",
            format_size(self.bytes_copied, BINARY),
            self.files_copied,
            self.bytes_copied as f64 / 1_000_000.0 / self.time_taken.as_secs_f64()
        )
    }
}

fn resolve_target_path(source: &Path, dest: &Path) -> PathBuf {
    if dest.is_dir() {
        dest.join(source.file_name().unwrap())
    } else {
        dest.to_path_buf()
    }
}

fn get_total_size(path: &Path) -> Result<u64, CopyError> {
    if path.is_file() {
        Ok(path.metadata()?.len())
    } else {
        let mut total = 0;
        for entry in WalkDir::new(path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                total += entry.metadata()?.len();
            }
        }
        Ok(total)
    }
}

fn copy_file(
    source: &Path,
    dest: &Path,
    pb: &ProgressBar,
    preserve_attrs: bool,
) -> io::Result<u64> {
    let mut copied = 0;
    let src_file = File::open(source)?;
    let dst_file = File::create(dest)?;

    let mut reader = BufReader::new(src_file);
    let mut writer = BufWriter::new(dst_file);
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let n = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        writer.write_all(&buffer[..n])?;
        copied += n as u64;
        pb.inc(n as u64);
    }

    writer.flush()?;

    if preserve_attrs {
        let metadata = source.metadata()?;
        fs::set_permissions(dest, metadata.permissions())?;
    }

    Ok(copied)
}

pub fn copy_with_progress(
    source: &Path,
    dest: &Path,
    options: &CopyOptions,
) -> Result<CopyStats, CopyError> {
    let start_time = std::time::Instant::now();
    let mut stats = CopyStats::new();

    // Handle source file/directory checks
    if source.is_dir() && !options.recursive {
        return Err(CopyError::IsADirectory(source.to_path_buf()));
    }

    // Calculate total size for progress bar
    let total_size = get_total_size(source)?;
    let multi = MultiProgress::new();
    let pb = multi.add(ProgressBar::new(total_size));
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .expect("Progress bar template error")
        .progress_chars("#>-"));

    if source.is_file() {
        // Copying a single file
        let target = resolve_target_path(source, dest);
        stats.bytes_copied = copy_file(source, &target, &pb, options.preserve_attrs)?;
        stats.files_copied = 1;
    } else if options.recursive {
        // Copying directory recursively
        let target_base = if dest.exists() && dest.is_dir() {
            dest.join(source.file_name().unwrap())
        } else {
            dest.to_path_buf()
        };

        for entry in WalkDir::new(source) {
            let entry = entry?;
            let path = entry.path();
            let relative = path.strip_prefix(source)
                .map_err(|e| CopyError::Other(e.into()))?;
            let target = target_base.join(relative);

            if entry.file_type().is_dir() {
                fs::create_dir_all(&target)?;
                stats.dirs_created += 1;
            } else if entry.file_type().is_file() {
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                stats.bytes_copied += copy_file(path, &target, &pb, options.preserve_attrs)?;
                stats.files_copied += 1;
            }
        }
    }

    stats.time_taken = start_time.elapsed();
    pb.finish_with_message("Copy completed!");

    Ok(stats)
}
