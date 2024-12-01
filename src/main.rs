mod day1;
mod inputs;

use std::{
    io::Write,
    process::{Command, Stdio},
};

use clap::Parser;
use inputs::get_data;
use wl_clipboard_rs::copy::{MimeType, Options, Source};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    day: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let res = match args.day {
        1 => day1::solve(get_data(args.day).await.unwrap().as_str()),
        _ => panic!("Day not implemented"),
    };

    let opts = Options::new();
    opts.copy(
        Source::Bytes(res.to_string().into_bytes().into()),
        MimeType::Autodetect,
    )
    .unwrap();

    copy_to_clipboard(&res);

    println!("{}", res);
}

fn copy_to_clipboard(text: &str) {
    let mut cmd = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start wl-copy");
    if let Some(child_stdin) = cmd.stdin.as_mut() {
        // Use `child_stdin` here without consuming `cmd.stdin`
        // Example:
        writeln!(child_stdin, "{}", text).expect("Failed to write to stdin");
    } else {
        panic!("Failed to open stdin");
    }

    // You can now use `cmd` safely afterward
    cmd.wait().expect("Failed to wait for wl-copy to finish");
}
