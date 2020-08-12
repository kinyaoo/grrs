use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn find_a_match() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "lorem ipsum\ndolor sit amet")?;

    let path = std::path::PathBuf::from(file.path());
    let pattern = String::from("lorem");
    let content = grrs::file_content(&path);
    let mut result = Vec::new();

    grrs::find_matches(content.unwrap(), &pattern, &mut result).ok();
    
    assert_eq!(result, b"lorem ipsum\n");

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "lorem ipsum\ndolor sit amet")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}
