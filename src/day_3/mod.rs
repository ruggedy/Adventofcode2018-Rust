// use std::cmp::Ordering;
use std::collections::{HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result as IoResult};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct Edge(u16, u16);

type Weight = u16;

pub fn calc_overlapping_materials() -> IoResult<u32> {
    let f = File::open("files/day_three_input.txt")?;
    let f = BufReader::new(f);
    let values_hash_map =
        f.lines()
            .into_iter()
            .fold(HashMap::new(), |mut acc: HashMap<Edge, Weight>, v| {
                let values = v.unwrap();

                let mut values = values
                    .split(|c| c == ',' || c == ':' || c == 'x' || c == '@' || c == '#')
                    .filter_map(|val| match val.trim().parse::<u16>() {
                        Ok(v) => Some(v),
                        _ => None,
                    });

                values.next().unwrap();
                let x = values.next().unwrap();
                let y = values.next().unwrap();
                let width = values.next().unwrap();
                let height = values.next().unwrap();


                for pos in 0..width * height {
                    let key = Edge(x + (pos % width), y + (pos / width));
                    *acc.entry(key).or_insert(0) += 1;
                }

                acc
            });

    Ok(values_hash_map
        .iter()
        .fold(0, |acc, (_, value)| if *value > 1 { acc + 1 } else { acc }))
}

pub fn find_valid_claim() -> IoResult<String> {

    Ok("".to_string())
}
