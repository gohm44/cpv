use clap::Parser;
use cpv::{copy_with_progress, CopyError, CopyOptions};
use std::path::PathBuf;
use std::process;

/// Modern file copy utility with progress visualization
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source file or directory
    #[arg(name = "SOURCE")]
    source: PathBuf,

    /// Destination file or directory
    #[arg(name = "DEST")]
    destination: PathBuf,

    /// Copy directories recursively
    #[arg(short = 'r', long = "recursive")]
    recursive: bool,

    /// Preserve attributes
    #[arg(short = 'p', long)]
    preserve: bool,

    /// Force overwrite
    #[arg(short = 'f', long)]
    force: bool,

    /// Verbose output
    #[arg(short = 'v', long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let options = CopyOptions {
        preserve_attrs: args.preserve,
        force: args.force,
        verbose: args.verbose,
        recursive: args.recursive,
    };

    match copy_with_progress(&args.source, &args.destination, &options) {
        Ok(stats) => {
            if options.verbose {
                println!("{}", stats.format_summary());
            }
        }
        Err(CopyError::NotADirectory(path)) => {
            eprintln!("cpv: {}: Not a directory", path.display());
            process::exit(1);
        }
        Err(CopyError::IsADirectory(path)) => {
            eprintln!(
                "cpv: {}: Is a directory (not copied, try using -r)",
                path.display()
            );
            process::exit(1);
        }
        Err(CopyError::Io(err)) => {
            eprintln!("cpv: {}", err);
            process::exit(1);
        }
        Err(err) => {
            eprintln!("cpv: {}", err);
            process::exit(1);
        }
    }
}
