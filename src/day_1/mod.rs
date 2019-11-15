use std::io::{BufReader, Result};
use std::io::prelude::*;
use std::fs::File;

pub fn calc_frequency() -> Result<i64> {
  let f = File::open("files/day_one_input.txt")?;
  let f = BufReader::new(f);

  let mut v: i64 = 0;

  for line in f.lines() {
    let current = v;
    let change = line.unwrap().parse::<i64>();

    match change {
      Ok(change) => {
        v = v + change;
        println!("- Current frequency {}, change of {}; resulting frequency {}", current, change, v);
      },
      Err(e) => {
        println!("- Current frequency {}, invalid frequency {}; no change", current, e);
      }
    }
  }

  Ok(v)
}
