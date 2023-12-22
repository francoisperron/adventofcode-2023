use std::{env, fs};

pub fn daily_input(day: u32) -> String {
    let session = env::var("AOC_SESSION").unwrap();

    let mut content = ureq::get(&format!("https://adventofcode.com/2023/day/{day}/input"))
        .set("Cookie", &format!("session={}", session))
        .call().unwrap()
        .into_string().unwrap();
    content.pop();
    content
}

pub fn daily_example(day: u32) -> String {
    fs::read_to_string(format!("src/day{:0>2}.input", day)).unwrap()
        .parse().unwrap()
}