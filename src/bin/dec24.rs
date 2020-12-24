// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(dead_code, unused_imports)]

use std::collections::BTreeSet;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::{IResult, Parser};

/// Hex coordinates {x, y} where x increases by 2 for horizontally adjacent
/// hexes.
// type Hex = (isize, isize);
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Hex {
    x: isize,
    y: isize,
}

pub fn main() {
    println!("24a: {}", solve_a());
    println!("24b: {}", solve_b());
}

fn solve_a() -> usize {
    solve_type_a(&load())
}

fn solve_type_a(s: &str) -> usize {
    let black = load_map(s);
    black.len()
}

fn load_map(s: &str) -> BTreeSet<Hex> {
    let mut black: BTreeSet<Hex> = BTreeSet::new();
    for line in parse(s) {
        let coord = reduce(&line);
        if !black.insert(coord.clone()) {
            black.remove(&coord);
        }
    }
    black
}

fn reduce(line: &[&str]) -> Hex {
    let mut x = 0;
    let mut y = 0;
    for d in line {
        match *d {
            "e" => x += 2,
            "w" => x -= 2,
            "ne" => {
                y += 1;
                x += 1
            }
            "nw" => {
                y += 1;
                x -= 1
            }
            "sw" => {
                y -= 1;
                x -= 1
            }
            "se" => {
                y -= 1;
                x += 1
            }
            _other => panic!(),
        }
    }
    Hex { x, y }
}

fn countchar(s: &str, c: char) -> isize {
    s.chars().filter(|x| *x == c).count() as isize
}

fn parse(s: &str) -> Vec<Vec<&str>> {
    try_parse(s).unwrap().1
}

fn try_parse(s: &str) -> IResult<&str, Vec<Vec<&str>>> {
    many1(terminated(
        many1(alt((
            tag("e"),
            tag("w"),
            tag("nw"),
            tag("ne"),
            tag("sw"),
            tag("se"),
        ))),
        newline,
    ))(s)
}

fn solve_b() -> usize {
    solve_type_b(&load())
}

fn neighbors(Hex { x, y }: &Hex) -> Vec<Hex> {
    vec![
        Hex { x: x + 2, y: *y },
        Hex { x: x - 2, y: *y },
        Hex { x: x + 1, y: y + 1 },
        Hex { x: x + 1, y: y - 1 },
        Hex { x: x - 1, y: y + 1 },
        Hex { x: x - 1, y: y - 1 },
    ]
}

fn solve_type_b(s: &str) -> usize {
    let mut black = load_map(s);
    dbg!(&black);
    for day in 0..100 {
        let interest: BTreeSet<Hex> = black
            .iter()
            .flat_map(|h| {
                let mut n = neighbors(h);
                n.push(h.clone());
                n
            })
            .collect();
        dbg!(interest.len());
        let mut newmap: BTreeSet<Hex> = BTreeSet::new();
        for h in interest {
            let bns = neighbors(&h).iter().filter(|n| black.contains(n)).count();
            let newstate = if black.contains(&h) {
                !(bns == 0 || bns > 2)
            } else {
                bns == 2
            };
            if newstate {
                let added = newmap.insert(h.clone());
                assert!(added, "{:?} somehow already present", h);
            }
        }
        black = newmap;
        println!("day {}: {}", day, black.len());
    }
    black.len()
}

fn load() -> String {
    std::fs::read_to_string("input/dec24.txt").unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {}

    #[test]
    fn solution_b() {}
}
