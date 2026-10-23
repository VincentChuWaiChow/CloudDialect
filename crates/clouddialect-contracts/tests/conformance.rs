#[path = "support/fixtures.rs"]
mod fixtures;

use std::{
    path::PathBuf,
    process::{Command, ExitCode},
};

fn run() -> Result<(), String> {
    let arguments: Vec<_> = std::env::args().skip(1).collect();
    if arguments.first().is_some_and(|arg| arg == "--fixture") {
        if arguments.len() != 2 {
            return Err("invalid internal fixture arguments".to_owned());
        }
        return fixtures::check_fixture(&PathBuf::from(&arguments[1]));
    }
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let mut prefix = None;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        if argument == "--fixture-root" {
            root = arguments
                .next()
                .ok_or("--fixture-root requires a directory")?
                .into();
        } else if argument.starts_with('-') || prefix.replace(argument.as_str()).is_some() {
            return Err(
                "expected one directory prefix and optional --fixture-root path".to_owned(),
            );
        }
    }
    let selected = fixtures::discover(&root, prefix)?;
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    for directory in &selected {
        let mut command = Command::new(&executable);
        command.arg("--fixture").arg(directory);
        fixtures::run_bounded(command, fixtures::FIXTURE_TIMEOUT)
            .map_err(|e| format!("{}: {e}", directory.display()))?;
        println!("PASS {}", directory.strip_prefix(&root).unwrap().display());
    }
    println!("{} conformance fixtures passed", selected.len());
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", error.chars().take(2048).collect::<String>());
            ExitCode::FAILURE
        }
    }
}
