use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result as IoResult};

pub fn calc_frequency() -> IoResult<i64> {
  let f = File::open("files/day_one_input.txt")?;
  let f = BufReader::new(f);

  let mut v: i64 = 0;

  for line in f.lines() {
    let change = line.unwrap().parse::<i64>();

    match change {
      Ok(change) => {
        v = v + change;
      }
      Err(_) => {
        continue;
      }
    }
  }

  Ok(v)
}

pub fn calc_multiple_frequency() -> IoResult<i64> {
  let f = File::open("files/day_one_input.txt")?;
  let f = BufReader::new(f);
  let mut changes = HashSet::new();
  let mut v: i64 = 0;
  let values = f
    .lines()
    .filter_map(|val| val.unwrap().trim().parse::<i64>().ok())
    .collect::<Vec<i64>>();

  for val in values.iter().cycle() {
    v = v + val;
    if changes.contains(&v) {
      break;
    }
    changes.insert(v);
  }

  Ok(v)
}
