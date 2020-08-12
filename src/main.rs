use std::io::{self};
use structopt::StructOpt;
use exitfailure::ExitFailure;


/// Search for a pattern in a file and display the lines that containt it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let args = Cli::from_args();
    let content = grrs::file_content(&args.path)?;

    grrs::find_matches(content, &args.pattern, &mut writer)?;

    Ok(())
}
