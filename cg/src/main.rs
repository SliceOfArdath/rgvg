use clap::{Parser, command, ArgGroup};
use std::path::PathBuf;
use std::process::{Command,Output,Stdio,Child};
use std::{io, fs};

mod command;

#[derive(Parser, Debug)]
#[command(author = "SliceOfArdath", version, about = "Find code, fast.", long_about = None)]
struct Args {
    /// The regular expression used for searching.
    #[arg(required=true,value_name="PATTERN")]
    pattern: String,
    /// The file or directory to search.
    #[arg(value_name="PATH")]
    file: Option<PathBuf>,
}

///Creates a command from a command string.
fn build(command: Vec<&str>) -> Command {
    let mut output = Command::new(command.get(0).expect("No command attached!"));

    for i in 1..command.len() {
        output.arg(command[i]);
    }
    return output;
}

///Call the first command in a call chain
fn begin(first: Vec<&str>) -> Child {
    return build(first).stdout(Stdio::piped()).spawn().expect("Failed command");
}
/// Links the first command's ouput to the second's input, then starts the second command.
fn link(first: Child, second: Vec<&str>) -> Child {
    //first.stdout(Stdio::piped());
    return build(second).stdin(first.stdout.unwrap()).stdout(Stdio::piped()).spawn().expect("Failed command");
}
///Finishes a call stack
fn finish(last: Child) -> Result<Output, io::Error> {
    return last.wait_with_output();
}



fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}