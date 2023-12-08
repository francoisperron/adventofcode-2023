#![allow(dead_code)]
use std::{env, fs};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub fn daily_input(day: u32) -> String {
    let session = env::var("AOC_SESSION").unwrap();

    ureq::get(&format!("https://adventofcode.com/2023/day/{day}/input"))
        .set("Cookie", &format!("session={}", session))
        .call().unwrap()
        .into_string().unwrap()
}

pub fn daily_example(day: u32) -> String {
    fs::read_to_string(format!("src/day{:0>2}.input", day)).unwrap()
        .parse().unwrap()
}