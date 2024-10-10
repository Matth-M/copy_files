use clap::{ArgAction, Parser};
use copypasta::{ClipboardContext, ClipboardProvider};
use std::path::Path;
#[derive(Parser)]
#[command(name = "copy_content")]
#[command(version = "0.1")]
#[command(about = "Copy contents of files selected files.", long_about = None)]
struct Cli {
    #[arg(action=ArgAction::Append)]
    files: Vec<String>,
}

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();

    let cli = Cli::parse();
    let files = cli.files;

    let mut paths: Vec<Box<Path>> = vec![];
    let mut msg = String::new();

    for file in files {
        let path = Path::new(&file);
        if path.exists() {
            let content = std::fs::read(path).unwrap_or("".into());
            let content = String::from_utf8(content).unwrap_or("".to_string());
            msg.push_str(&content);
            paths.push(path.into());
        }
    }

    ctx.set_contents(msg.to_owned()).unwrap();
    loop {}
}
