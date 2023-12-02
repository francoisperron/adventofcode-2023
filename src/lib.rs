#![allow(dead_code)]
use std::env;

mod day01;
mod day02;

pub fn daily_input(day: u32) -> String {
    let session = env::var("AOC_SESSION").unwrap();

    reqwest::blocking::Client::new()
        .get(format!("https://adventofcode.com/2023/day/{day}/input"))
        .header("Cookie", format!("session={}", session))
        .send().unwrap()
        .text().unwrap()
}