#[macro_use]
extern crate criterion;

use criterion::Criterion;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufWriter};
use std::collections::HashMap;
use std::process::exit;

fn bench(s: &str) {
    let mut file = File::open(s).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout);
    let mut program = vec![""; input.len()];

    for line in input.iter() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let n = split[0].parse::<usize>().unwrap();
        program[n / 10 - 1] = line;
    }

    for line in program {
        let split: Vec<&str> = line.split_whitespace().collect();
        match split[0] {
            "IF" => {},
            "LET" => {},
            "PRINT" => {},
            "PRINTLN" => {},
            _ => {},
        }
    }
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("bench", |b| {
        b.iter(|| bench("F:\\Dev\\Rust\\kattis\\test\\basicinterpreter\\1.in"))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);