mod inputs;
mod solutions;

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
        1 => solutions::day1::solve(get_data(args.day).await.unwrap().as_str()),
        2 => solutions::day2::solve(get_data(args.day).await.unwrap().as_str()),
        3 => solutions::day3::solve(get_data(args.day).await.unwrap().as_str()),
        4 => solutions::day4::solve(get_data(args.day).await.unwrap().as_str()),
        5 => solutions::day5::solve(get_data(args.day).await.unwrap().as_str()),
        6 => solutions::day6::solve(get_data(args.day).await.unwrap().as_str()),
        7 => solutions::day7::solve(get_data(args.day).await.unwrap().as_str()),
        8 => solutions::day8::solve(get_data(args.day).await.unwrap().as_str()),
        9 => solutions::day9::solve(get_data(args.day).await.unwrap().as_str()),
        10 => solutions::day10::solve(get_data(args.day).await.unwrap().as_str()),
        11 => solutions::day11::solve(get_data(args.day).await.unwrap().as_str()),
        12 => solutions::day12::solve(get_data(args.day).await.unwrap().as_str()),
        13 => solutions::day13::solve(get_data(args.day).await.unwrap().as_str()),
        14 => solutions::day14::solve(get_data(args.day).await.unwrap().as_str()),
        15 => solutions::day15::solve(get_data(args.day).await.unwrap().as_str()),
        16 => solutions::day16::solve(get_data(args.day).await.unwrap().as_str()),
        17 => solutions::day17::solve(get_data(args.day).await.unwrap().as_str()),
        18 => solutions::day18::solve(get_data(args.day).await.unwrap().as_str()),
        19 => solutions::day19::solve(get_data(args.day).await.unwrap().as_str()),
        20 => solutions::day20::solve(get_data(args.day).await.unwrap().as_str()),
        21 => solutions::day21::solve(get_data(args.day).await.unwrap().as_str()),
        22 => solutions::day22::solve(get_data(args.day).await.unwrap().as_str()),
        23 => solutions::day23::solve(get_data(args.day).await.unwrap().as_str()),
        24 => solutions::day24::solve(get_data(args.day).await.unwrap().as_str()),
        25 => solutions::day25::solve(get_data(args.day).await.unwrap().as_str()),
        26 => solutions::day26::solve(get_data(args.day).await.unwrap().as_str()),
        27 => solutions::day27::solve(get_data(args.day).await.unwrap().as_str()),
        28 => solutions::day28::solve(get_data(args.day).await.unwrap().as_str()),
        29 => solutions::day29::solve(get_data(args.day).await.unwrap().as_str()),
        30 => solutions::day30::solve(get_data(args.day).await.unwrap().as_str()),
        31 => solutions::day31::solve(get_data(args.day).await.unwrap().as_str()),
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
