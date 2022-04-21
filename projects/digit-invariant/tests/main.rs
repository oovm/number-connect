#![allow(dead_code)]
use narcissistic::narcissistic_numbers;
use num::BigUint;
use std::str::FromStr;

#[test]
fn narcissistic() {
    for (i, n) in narcissistic_numbers().enumerate() {
        println!("#{}: {}", i + 1, n);
    }
}

fn print_int(list: &[&str]) {
    println!("[");
    for i in list {
        println!("BigUint::from_bytes_le(&{:?}),", BigUint::from_str(i).unwrap().to_bytes_le());
    }
    println!("]");
}