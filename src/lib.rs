use std::io::{BufReader, Write};
use std::fs::File;
use std::io::prelude::*;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::path::{PathBuf};

pub fn find_matches(content: BufReader<File>, pattern: &String, mut writer: impl Write) -> Result<(), ExitFailure> {
    for line in content.lines() {
        let l = match_pattern(line.unwrap(), &pattern);
        if !l.is_empty() {
            writeln!(writer, "{}", l)
                .with_context(|_| format!("Could not write to stdout"))?;
        }
    }

    Ok(())
}

pub fn file_content(path: &PathBuf) -> Result<BufReader<File>, ExitFailure> {
    let file = File::open(path)
        .with_context(|_| format!("Could not read file `{}`", path.display()))?;
    let content = BufReader::new(file);

    Ok(content)
}

pub fn match_pattern(line: String, pattern: &String) -> String {
    match &line {
        s if s.contains(pattern) => line,
        _ => String::new(),
    }
}
