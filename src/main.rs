use std::env;
use std::io::BufRead;

mod day3;
mod day4;

fn main() {
    let args: Vec<_> = env::args().collect();
    let day = &args[1];
    let part = &args[2];

    let stdin = std::io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();

    let result = match day.as_str() {
        "3" => match part.as_str() {
            "1" => day3::part1(&lines),
            "2" => day3::part2(&lines),
            _ => panic!("invalid part: {}", part),
        },
        "4" => match part.as_str() {
            "1" => day4::part1(&lines),
            "2" => day4::part2(&lines),
            _ => panic!("invalid part: {}", part),
        },
        _ => panic!("invalid day: {}", day),
    };

    println!("{}", result);
}
