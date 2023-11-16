// https://codeforces.com/gym/103470/problem/M
// Will get Time Limit Exceeded if using println!() instead of writeln!().
use std::{
    collections::VecDeque,
    io::{BufWriter, Stdout, Write},
};

pub struct IO {
    buffer: VecDeque<String>,
    out: BufWriter<Stdout>,
}
impl IO {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
            out: BufWriter::new(std::io::stdout()),
        }
    }
    pub fn write<T>(&mut self, s: T)
    where
        T: std::fmt::Display,
    {
        write!(self.out, "{}", s).expect("Failed to write");
    }
    pub fn writeln<T>(&mut self, s: T)
    where
        T: std::fmt::Display,
    {
        writeln!(self.out, "{}", s).expect("Failed to write");
    }
    pub fn next<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                return token.parse().expect("Failed parse");
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn solve(io: &mut IO) {
    let n: usize = io.next();
    let a: Vec<i64> = (0..n).map(|_| io.next()).collect();
    if n == 1 {
        io.writeln(a[0]);
        return;
    }
    let a_max = *a.iter().max().unwrap();
    let a_min = *a.iter().min().unwrap();
    let a_abs_sum = a.iter().map(|x| x.abs()).sum::<i64>();
    if a_max < 0 {
        io.writeln(a_abs_sum - a_max.abs() * 2);
        return;
    }
    if a_min > 0 {
        io.writeln(a_abs_sum - a_min * 2);
        return;
    }
    io.writeln(a_abs_sum);
}

fn main() {
    let mut io = IO::new();
    let groups: usize = io.next();
    for _ in 0..groups {
        solve(&mut io);
    }
}
