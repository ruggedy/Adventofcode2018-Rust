use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result as IoResult};
// use

pub fn calc_checksum() -> IoResult<u64> {
    let f = File::open("files/day_two_input.txt")?;
    let f = BufReader::new(f);

    let lines = f.lines().map(|v| v.unwrap()).collect::<Vec<String>>();

    let checksum = lines.iter().fold((0, 0), |acc, next| {
        let mut tuple = (0, 0);
        let mut map: HashMap<u8, u64> = HashMap::new();
        for (_, &item) in next.as_bytes().iter().enumerate() {
            if map.contains_key(&item) {
                let count = map.get_mut(&item);
                *count.unwrap() += 1;
            } else {
                map.insert(item, 1);
            }
        }

        for (_, val) in map.iter() {
            if tuple.0 == 1 && tuple.1 == 1 {
                break;
            }

            match val {
                2 => {
                    tuple.0 = 1;
                }
                3 => {
                    tuple.1 = 1;
                }
                _ => continue,
            }
        }

        (acc.0 + tuple.0, acc.1 + tuple.1)
    });

    Ok(checksum.0 * checksum.1)
}

pub fn find_boxes() -> IoResult<String> {
    let f = File::open("files/day_two_input.txt")?;
    let f = BufReader::new(f);

    let lines = f
        .lines()
        .map(|v| v.unwrap().into_bytes())
        .collect::<Vec<Vec<u8>>>();

    // let mut paired: (Vec<u8>, Vec<u8>) = (vec![], vec![]);
    let mut found = String::new();

    for idx in 0..lines.len() {
        let curr_item = &lines[idx];
        for next_idx in idx + 1..lines.len() {
            let next_item = &lines[next_idx];

            let valid_id = curr_item
                .iter()
                .zip(next_item)
                .into_iter()
                .enumerate()
                .try_fold(vec![], |mut acc, (idx, (val1, val2))| {
                    if val1 == val2 {
                        acc.push(*val1);
                    }

                    if idx + 1 - acc.len() == 2 {
                        None
                    } else {
                        Some(acc)
                    }
                });

            if let Some(v) = valid_id {
                unsafe {
                    found = String::from_utf8_unchecked(v);
                }
                break;
            } else {
                continue;
            }
        }

        if !found.is_empty() {
            break;
        }
    }

    Ok(found)
}
