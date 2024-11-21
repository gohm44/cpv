use cpv::{copy_with_progress, CopyOptions};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_test_file(dir: &TempDir, name: &str, content: &[u8]) -> PathBuf {
    let file_path = dir.path().join(name);
    let mut file = File::create(&file_path).unwrap();
    file.write_all(content).unwrap();
    file_path
}

#[test]
fn test_copy_to_nonexistent_directory() {
    let temp = TempDir::new().unwrap();
    let source = create_test_file(&temp, "source.txt", b"test");
    let dest = temp.path().join("nonexistent").join("dest.txt");

    let options = CopyOptions {
        preserve_attrs: false,
        force: false,
        verbose: false,
        recursive: false,
    };

    let result = copy_with_progress(&source, &dest, &options);
    assert!(result.is_err());
}

#[test]
fn test_copy_large_file() {
    let temp = TempDir::new().unwrap();
    let source = create_test_file(&temp, "large.bin", &vec![0u8; 1024 * 1024]); // 1MB file
    let dest = temp.path().join("large_copy.bin");

    let options = CopyOptions {
        preserve_attrs: false,
        force: false,
        verbose: true,
        recursive: false,
    };

    let result = copy_with_progress(&source, &dest, &options);
    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.bytes_copied, 1024 * 1024);
}

#[test]
fn test_nested_directory_copy() {
    let temp = TempDir::new().unwrap();
    let source_dir = temp.path().join("source");
    fs::create_dir(&source_dir).unwrap();
    fs::create_dir(source_dir.join("subdir")).unwrap();
    create_test_file(&temp, "source/file1.txt", b"test1");
    create_test_file(&temp, "source/subdir/file2.txt", b"test2");

    let dest_dir = temp.path().join("dest");

    let options = CopyOptions {
        preserve_attrs: false,
        force: false,
        verbose: true,
        recursive: true,
    };

    let result = copy_with_progress(&source_dir, &dest_dir, &options);
    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.files_copied, 2);
    assert!(dest_dir.join("file1.txt").exists());
    assert!(dest_dir.join("subdir/file2.txt").exists());
}
