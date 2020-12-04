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

//! Solution to https://adventofcode.com/2020/day/3.

pub fn main() {
    println!("04a: {}", solve_a());
    println!("04b: {}", solve_b());
}

fn solve_a() -> usize {
    let input = std::fs::read_to_string("input/dec04.txt").unwrap();
    let mut valid = 0;
    for para in input.split("\n\n") {
        let fields: Vec<String> = para
            .split_whitespace()
            .map(|s| s.split(':').take(1).next().unwrap().to_owned())
            .collect();
        println!("{:#?}", fields);
        if fields.len() == 8 || (fields.len() == 7 && !fields.contains(&"cid".to_owned())) {
            valid += 1;
        }
    }
    valid
}

fn solve_b() -> usize {
    std::fs::read_to_string("input/dec04.txt")
        .unwrap()
        .split("\n\n")
        .filter(|p| is_valid(p))
        .count()
}

fn is_valid(para: &str) -> bool {
    let kv: Vec<(&str, &str)> = para
        .split_whitespace()
        .map(|s| {
            let mut parts = s.split(':');
            let k = parts.next().unwrap();
            let v = parts.next().unwrap();
            (k, v)
        })
        .collect();
    println!("{:?}", kv);
    match kv.len() {
        0..=6 => {
            println!("too short");
            return false;
        }
        7 if kv.iter().any(|(k, _)| *k == "cid") => {
            println!("len 7 and has cid");
            return false;
        }
        7 => (),
        8 => (),
        _more => {
            println!("too long");
            return false;
        }
    };
    println!("length ok");
    for (k, v) in kv {
        println!("check {:?} {:?}", k, v);
        let ok = match k {
            "byr" => v.len() == 4 && v >= "1920" && v <= "2002",
            "iyr" => v.len() == 4 && v >= "2010" && v <= "2020",
            "eyr" => v.len() == 4 && v >= "2020" && v <= "2030",
            "hgt" => {
                if let Some(cm) = v.strip_suffix("cm") {
                    cm >= "150" && cm <= "193"
                } else if let Some(inch) = v.strip_suffix("in") {
                    dbg!(&inch);
                    inch >= "59" && inch <= "76"
                } else {
                    false
                }
            }
            "hcl" => {
                let chars: Vec<char> = v.chars().collect();
                chars.len() == 7
                    && chars[0] == '#'
                    && chars[1..].iter().all(char::is_ascii_hexdigit)
            }
            "ecl" => match v {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => {
                    println!("bad ecl");
                    false
                }
            },
            "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
            _ => true,
        };
        if ok {
            println!("field is ok");
        } else {
            println!("field validation failed");
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {}

    #[test]
    fn solution_b() {}

    #[test]
    fn invalid_examples() {
        let data = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(data.split("\n\n").count(), 4);
        // assert!(data.split("\n\n").all(|para| !is_valid(para)));
    }

    #[test]
    fn valid_examples() {
        let data = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert!(data.split("\n\n").all(|para| is_valid(para)));
    }
}