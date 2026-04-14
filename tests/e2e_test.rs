use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::tempdir;

// Real-world scenario with all features and flags.
#[test]
fn test_complete_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let home_path = dir.path();

    let downloads_path = home_path.join("Downloads");
    fs::create_dir_all(&downloads_path)?;

    fs::write(downloads_path.join("image.jpeg"), "image")?;
    fs::write(downloads_path.join("image_2.jpeg"), "image 2")?;
    fs::write(downloads_path.join("report.pdf"), "report")?;
    fs::write(downloads_path.join("invalid_file.jph"), "invalid file")?;
    fs::write(downloads_path.join(".zshrc.exe"), "system file")?;

    fs::write(downloads_path.join("not_movable.exe"), "not movable")?;

    let dest_path = home_path;
    let dest_pics_path = dest_path.join("Pictures");
    let dest_installers_path = dest_path.join("Installers");
    fs::create_dir_all(&dest_pics_path)?;
    fs::create_dir_all(&dest_installers_path)?;
    fs::write(dest_pics_path.join("image_2.jpeg"), "image 2")?;

    let mut dir_perms = fs::metadata(&dest_installers_path)?.permissions();
    dir_perms.set_mode(0o444);
    fs::set_permissions(&dest_installers_path, dir_perms)?;

    let mut init_cmd = Command::cargo_bin("forg")?;
    init_cmd.env("HOME", home_path).arg("init");
    init_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Config created at"));

    let mut execution_cmd = Command::cargo_bin("forg")?;
    execution_cmd
        .env("HOME", home_path)
        .arg("Downloads")
        .arg("--ignore-case");
    execution_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Execution Mode: Complete"))
        .stdout(predicate::str::contains(
            "dry-run=false, ignore-case=true, allow-hidden=false",
        ))
        .stdout(predicate::str::contains("Total Files : 6"))
        .stdout(predicate::str::contains("Total Matched: 4"))
        .stdout(predicate::str::contains("Moved Successfully: 2"))
        .stdout(predicate::str::contains("Skipped (Conflict): 1"))
        .stdout(predicate::str::contains("Failed (Errors): 1"))
        .stdout(predicate::str::contains("--- Errors ---"));

    Ok(())
}
