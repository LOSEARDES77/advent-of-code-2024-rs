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
    #[arg(default_value = "1")]
    part: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let data = get_data(args.day).await.unwrap();
    let res = match args.day {
        1 => match args.part {
            1 => solutions::day1::solve(&data),
            2 => solutions::day1::solve_p2(&data),
            _ => panic!("No such part"),
        },
        2 => match args.part {
            1 => solutions::day2::solve(&data),
            2 => solutions::day2::solve_p2(&data),
            _ => panic!("No such part"),
        },
        3 => match args.part {
            1 => solutions::day3::solve(&data),
            2 => solutions::day3::solve_p2(&data),
            _ => panic!("No such part"),
        },
        4 => match args.part {
            1 => solutions::day4::solve(&data),
            2 => solutions::day4::solve_p2(&data),
            _ => panic!("No such part"),
        },
        5 => match args.part {
            1 => solutions::day5::solve(&data),
            2 => solutions::day5::solve_p2(&data),
            _ => panic!("No such part"),
        },
        6 => match args.part {
            1 => solutions::day6::solve(&data),
            2 => solutions::day6::solve_p2(&data),
            _ => panic!("No such part"),
        },
        7 => match args.part {
            1 => solutions::day7::solve(&data),
            2 => solutions::day7::solve_p2(&data),
            _ => panic!("No such part"),
        },
        8 => match args.part {
            1 => solutions::day8::solve(&data),
            2 => solutions::day8::solve_p2(&data),
            _ => panic!("No such part"),
        },
        9 => match args.part {
            1 => solutions::day9::solve(&data),
            2 => solutions::day9::solve_p2(&data),
            _ => panic!("No such part"),
        },
        10 => match args.part {
            1 => solutions::day10::solve(&data),
            2 => solutions::day10::solve_p2(&data),
            _ => panic!("No such part"),
        },
        11 => match args.part {
            1 => solutions::day11::solve(&data),
            2 => solutions::day11::solve_p2(&data),
            _ => panic!("No such part"),
        },
        12 => match args.part {
            1 => solutions::day12::solve(&data),
            2 => solutions::day12::solve_p2(&data),
            _ => panic!("No such part"),
        },
        13 => match args.part {
            1 => solutions::day13::solve(&data),
            2 => solutions::day13::solve_p2(&data),
            _ => panic!("No such part"),
        },
        14 => match args.part {
            1 => solutions::day14::solve(&data),
            2 => solutions::day14::solve_p2(&data),
            _ => panic!("No such part"),
        },
        15 => match args.part {
            1 => solutions::day15::solve(&data),
            2 => solutions::day15::solve_p2(&data),
            _ => panic!("No such part"),
        },
        16 => match args.part {
            1 => solutions::day16::solve(&data),
            2 => solutions::day16::solve_p2(&data),
            _ => panic!("No such part"),
        },
        17 => match args.part {
            1 => solutions::day17::solve(&data),
            2 => solutions::day17::solve_p2(&data),
            _ => panic!("No such part"),
        },
        18 => match args.part {
            1 => solutions::day18::solve(&data),
            2 => solutions::day18::solve_p2(&data),
            _ => panic!("No such part"),
        },
        19 => match args.part {
            1 => solutions::day19::solve(&data),
            2 => solutions::day19::solve_p2(&data),
            _ => panic!("No such part"),
        },
        20 => match args.part {
            1 => solutions::day20::solve(&data),
            2 => solutions::day20::solve_p2(&data),
            _ => panic!("No such part"),
        },
        21 => match args.part {
            1 => solutions::day21::solve(&data),
            2 => solutions::day21::solve_p2(&data),
            _ => panic!("No such part"),
        },
        22 => match args.part {
            1 => solutions::day22::solve(&data),
            2 => solutions::day22::solve_p2(&data),
            _ => panic!("No such part"),
        },
        23 => match args.part {
            1 => solutions::day23::solve(&data),
            2 => solutions::day23::solve_p2(&data),
            _ => panic!("No such part"),
        },
        24 => match args.part {
            1 => solutions::day24::solve(&data),
            2 => solutions::day24::solve_p2(&data),
            _ => panic!("No such part"),
        },
        25 => match args.part {
            1 => solutions::day25::solve(&data),
            2 => solutions::day25::solve_p2(&data),
            _ => panic!("No such part"),
        },
        26 => match args.part {
            1 => solutions::day26::solve(&data),
            2 => solutions::day26::solve_p2(&data),
            _ => panic!("No such part"),
        },
        27 => match args.part {
            1 => solutions::day27::solve(&data),
            2 => solutions::day27::solve_p2(&data),
            _ => panic!("No such part"),
        },
        28 => match args.part {
            1 => solutions::day28::solve(&data),
            2 => solutions::day28::solve_p2(&data),
            _ => panic!("No such part"),
        },
        29 => match args.part {
            1 => solutions::day29::solve(&data),
            2 => solutions::day29::solve_p2(&data),
            _ => panic!("No such part"),
        },
        30 => match args.part {
            1 => solutions::day30::solve(&data),
            2 => solutions::day30::solve_p2(&data),
            _ => panic!("No such part"),
        },
        31 => match args.part {
            1 => solutions::day31::solve(&data),
            2 => solutions::day31::solve_p2(&data),
            _ => panic!("No such part"),
        },

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
