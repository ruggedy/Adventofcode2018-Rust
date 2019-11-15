use std::io::{BufReader, Result};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;


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

pub fn calc_multiple_frequency() -> Result<i64> {
  let f = File::open("files/day_one_input.txt")?;
  let f = BufReader::new(f);
  let mut changes = HashSet::new();
  let mut v: i64 = 0;
  let mut found = false;
  let mut values: Vec<i64> = vec![];
  // calc values
  for line in f.lines() {
    let int_from_str = line.unwrap().parse::<i64>().unwrap();
    values.push(int_from_str);
  }

  while !found {
    for val in values.clone().into_iter() {
      v = v + val;
      if changes.contains(&v) {
        found = true;
        break;
      }
      changes.insert(v);
    }
  }

  Ok(v)
}
