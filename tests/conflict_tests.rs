use std::fs;
use std::process::Command;
use tempfile::tempdir;

fn get_binary_path() -> String {
    env!("CARGO_BIN_EXE_forg").to_string()
}

#[test]
fn test_conflict_skip() {
    let dir = tempdir().unwrap();
    let home = dir.path().join("home");
    let forg_dir = home.join(".forg");
    fs::create_dir_all(&forg_dir).unwrap();

    // Create source file
    let source_dir = home.join("source");
    fs::create_dir_all(&source_dir).unwrap();
    let source_file = source_dir.join("sample.txt");
    fs::write(&source_file, "new content").unwrap();

    // Create destination file with conflict
    let dest_dir = home.join("dest");
    fs::create_dir_all(&dest_dir).unwrap();
    let dest_file = dest_dir.join("sample.txt");
    fs::write(&dest_file, "old content").unwrap();

    // Run forg with --on-conflict skip (default)
    let status = Command::new(get_binary_path())
        .env("HOME", &home)
        .arg("source")
        .arg("-p").arg(".*")
        .arg("-t").arg("dest")
        .arg("--on-conflict").arg("skip")
        .status()
        .expect("failed to execute process");

    assert!(status.success());
    // In skip mode, file should still be in source and dest should remain unchanged
    assert!(source_file.exists());
    assert_eq!(fs::read_to_string(&dest_file).unwrap(), "old content");
}

#[test]
fn test_conflict_replace() {
    let dir = tempdir().unwrap();
    let home = dir.path().join("home");
    let forg_dir = home.join(".forg");
    fs::create_dir_all(&forg_dir).unwrap();

    // Create source file
    let source_dir = home.join("source");
    fs::create_dir_all(&source_dir).unwrap();
    let source_file = source_dir.join("sample.txt");
    fs::write(&source_file, "new content").unwrap();

    // Create destination file with conflict
    let dest_dir = home.join("dest");
    fs::create_dir_all(&dest_dir).unwrap();
    let dest_file = dest_dir.join("sample.txt");
    fs::write(&dest_file, "old content").unwrap();

    // Run forg with --on-conflict replace
    let status = Command::new(get_binary_path())
        .env("HOME", &home)
        .arg("source")
        .arg("-p").arg(".*")
        .arg("-t").arg("dest")
        .arg("--on-conflict").arg("replace")
        .status()
        .expect("failed to execute process");

    assert!(status.success());
    // In replace mode, source file should be moved, old dest becomes .bak
    assert!(!source_file.exists());
    assert_eq!(fs::read_to_string(&dest_file).unwrap(), "new content");
    let bak_file = dest_dir.join("sample.txt.bak");
    assert!(bak_file.exists());
    assert_eq!(fs::read_to_string(&bak_file).unwrap(), "old content");
}

#[test]
fn test_conflict_versioned() {
    let dir = tempdir().unwrap();
    let home = dir.path().join("home");
    let forg_dir = home.join(".forg");
    fs::create_dir_all(&forg_dir).unwrap();

    // Create source file
    let source_dir = home.join("source");
    fs::create_dir_all(&source_dir).unwrap();
    let source_file = source_dir.join("sample.txt");
    fs::write(&source_file, "new content").unwrap();

    // Create destination file with conflict
    let dest_dir = home.join("dest");
    fs::create_dir_all(&dest_dir).unwrap();
    let dest_file = dest_dir.join("sample.txt");
    fs::write(&dest_file, "old content").unwrap();

    // Run forg with --on-conflict versioned
    let status = Command::new(get_binary_path())
        .env("HOME", &home)
        .arg("source")
        .arg("-p").arg(".*")
        .arg("-t").arg("dest")
        .arg("--on-conflict").arg("versioned")
        .status()
        .expect("failed to execute process");

    assert!(status.success());
    // In versioned mode, source file should be moved to a new name
    assert!(!source_file.exists());
    assert!(dest_file.exists()); // Original stays
    let versioned_file = dest_dir.join("sample_v1.txt");
    assert!(versioned_file.exists());
    assert_eq!(fs::read_to_string(&versioned_file).unwrap(), "new content");
}
