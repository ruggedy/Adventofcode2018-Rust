// use std::cmp::Ordering;
use std::cmp::{self, Ordering, PartialOrd};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result as IoResult};

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Hash, Default)]
struct Edge(u16, u16);

#[derive(Eq, PartialOrd, Clone, Hash, Debug, Default)]
struct Claim {
    id: u16,
    width: u16,
    height: u16,
    starting_point: Edge,
}

impl PartialEq for Claim {
    fn eq(&self, other: &Self) -> bool {
        self.starting_point == other.starting_point
    }
}

impl Ord for Claim {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.starting_point.0 + self.width,
            self.starting_point.1 + self.height,
        )
            .cmp(
                &((
                    other.starting_point.0 + other.width,
                    other.starting_point.1 + other.height,
                )),
            )
    }
}

pub fn calc_overlapping_materials() -> IoResult<()> {
    let f = File::open("files/day_three_input.txt")?;
    let f = BufReader::new(f);
    let mut claim_hash_set: BTreeSet<Claim> = BTreeSet::new();
    let mut max_grid_length = 0;
    for line in f.lines().into_iter() {
        let values = line.unwrap();

        let mut values = values
            .split(|c| c == ',' || c == ':' || c == 'x' || c == '@' || c == '#')
            .filter_map(|val| match val.trim().parse::<u16>() {
                Ok(v) => Some(v),
                _ => None,
            });

        let id = values.next().unwrap();
        let x = values.next().unwrap();
        let y = values.next().unwrap();
        let width = values.next().unwrap();
        let height = values.next().unwrap();

        let claim = Claim {
            id,
            starting_point: Edge(x, y),
            width,
            height,
        };

        max_grid_length = cmp::max(max_grid_length, cmp::max(x + width, y + height));
        claim_hash_set.insert(claim);
    }

    let mut grid: Vec<Vec<Vec<&Claim>>> =
        vec![vec![vec![]; max_grid_length as usize]; max_grid_length as usize];

    let mut potential_correct_claim: Vec<Claim> = Vec::new();
    let mut count = 0;

    let mut peekable_claim_iter = claim_hash_set.iter().peekable();

    while let Some(claim) = peekable_claim_iter.next() {
        let mut should_peek = true;

        for y in (claim.starting_point.1) as usize..(claim.starting_point.1 + claim.height) as usize
        {
            for x in
                (claim.starting_point.0) as usize..(claim.starting_point.0 + claim.width) as usize
            {
                if let Some(row) = grid.get_mut(y) {
                    if let Some(column) = row.get_mut(x) {
                        column.push(claim);

                        let len = column.len();

                        if len == 2 {
                            count += 1;
                        }

                        if len >= 2 {
                            should_peek = false;
                        }
                    }
                }
            }
        }

        if should_peek {
            if let Some(next_claim) = peekable_claim_iter.peek() {
                let overlap = claim.starting_point.0 + claim.width > next_claim.starting_point.1
                    && claim.starting_point.1 + claim.height > next_claim.starting_point.1;
                if !overlap {
                    potential_correct_claim.push(claim.clone());
                }
            }
        }
    }

    println!("overlapping square inches {}", count);

    'outer: for pot_claim in potential_correct_claim.iter() {
        for y in (pot_claim.starting_point.1) as usize
            ..(pot_claim.starting_point.1 + pot_claim.height) as usize
        {
            for x in (pot_claim.starting_point.0) as usize
                ..(pot_claim.starting_point.0 + pot_claim.width) as usize
            {
                if grid[y][x].len() > 1 {
                    continue 'outer;
                }
            }
        }

        println!("this is correct claim id {} ", pot_claim.id);
        break;
    }

    Ok(())
}
