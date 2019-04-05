#![allow(dead_code)]

use std::io::{self, BufRead, Read};
use std::str::SplitWhitespace;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::f32::consts::PI;

fn oddities() {
    let mut n: String = String::new();
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");
    let n: u8 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    let mut vec: Vec<i8> = Vec::with_capacity(n as usize);
    for _i in 0..n {
        let mut n: String = String::new();
        io::stdin().read_line(&mut n)
            .expect("Failed to read line");
        vec.push(match n.trim().parse() {
            Ok(num) => num,
            Err(_) => return,
        });
    }
    for x in &vec {
        if x % 2 == 0 {
            println!("{} is even", x);
        } else {
            println!("{} is odd", x);
        }
    }
}

fn faktor() {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s)
        .expect("Failed to read line");
    let s: &str = &s[..];
    let s: SplitWhitespace = s.split_whitespace();
    let s: Vec<&str> = s.collect();
    let a: u16 = match s[0].trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    let b: u16 = match s[1].trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    println!("{}", (b - 1) * a + 1);
}

fn hissingmicrophone() {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s)
        .expect("Failed to read line");
    if s.contains("ss") {
        println!("hiss");
    } else {
        println!("no hiss");
    }
}

fn fizzbuzz() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let input: Vec<&str> = input.split_whitespace().collect();
    let x: usize = match input[0].trim_end().parse() {
        Ok(t) => t,
        Err(_) => return,
    };
    let y: u8 = match input[1].trim_end().parse() {
        Ok(t) => t,
        Err(_) => return,
    };
    let n: u8 = match input[2].trim_end().parse() {
        Ok(t) => t,
        Err(_) => return,
    };
    let n: u8 = n + 1;
    for i in 1..n {
        let is_fizz: u8 = i % x as u8;
        let is_buzz: u8 = i % y as u8;
        if is_fizz == 0 && is_buzz == 0 {
            println!("{}", "FizzBuzz");
        } else if is_fizz == 0 {
            println!("{}", "Fizz");
        } else if is_buzz == 0 {
            println!("{}", "Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn cold() {
    {
        let mut n = String::new();
        io::stdin().read_line(&mut n)
            .expect("Failed to read line");
    }
    let mut temperatures = String::new();
    io::stdin().read_line(&mut temperatures)
        .expect("Failed to read line");
    let temperatures = temperatures.split_whitespace();

    let mut i: u8 = 0;
    for temperature in temperatures {
        let t: i32 = match temperature.parse() {
            Ok(t) => t,
            Err(_) => return,
        };
        if t < 0 {
            i += 1;
        }
    }
    println!("{}", i);
}

fn bijele() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let input: Vec<&str> = input.split_whitespace().collect();
    {
        let kings: i8 = match input[0].parse() {
            Ok(t) => t,
            Err(_) => return,
        };
        print!("{} ", 1 - kings);
    }
    {
        let queens: i8 = match input[1].parse() {
            Ok(t) => t,
            Err(_) => return,
        };
        print!("{} ", 1 - queens);
    }
    {
        let rooks: i8 = match input[2].parse() {
            Ok(t) => t,
            Err(_) => return,
        };
        print!("{} ", 2 - rooks);
    }
    {
        let bishops: i8 = match input[3].parse() {
            Ok(t) => t,
            Err(_) => return,
        };
        print!("{} ", 2 - bishops);
    }
    {
        let knights: i8 = match input[4].parse() {
            Ok(t) => t,
            Err(_) => return,
        };
        print!("{} ", 2 - knights);
    }
    {
        let pawns: i8 = match input[5].parse() {
            Ok(t) => t,
            Err(_) => return,
        };
        print!("{}", 8 - pawns);
    }
}

fn nastyhacks() {
    let mut n: String = String::new();
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");
    let n: usize = match n.trim_end().parse() {
        Ok(t) => t,
        Err(_) => return,
    };
    let mut cases: Vec<String> = Vec::with_capacity(n);
    for _i in 0..n {
        let mut s: String = String::new();
        io::stdin().read_line(&mut s)
            .expect("Failed to read line");
        cases.push(s.trim_end().to_string());
    }
    for case in cases {
        let case: Vec<&str> = case.split_whitespace().collect();
        let r: i32 = match case[0].parse() {
            Ok(t) => t,
            Err(_) => return,
        };
        let mut ec: i32;
        {
            let e: i32 = match case[1].parse() {
                Ok(t) => t,
                Err(_) => return,
            };
            let c: i32 = match case[2].parse() {
                Ok(t) => t,
                Err(_) => return,
            };
            ec = e - c;
        }
        if r < ec {
            println!("{}", "advertise");
        } else if r > ec {
            println!("{}", "do not advertise");
        } else {
            println!("{}", "does not matter");
        }
    }
}

fn isithalloween() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let date: Vec<&str> = input.split_whitespace().collect();
    if (date[0] == "OCT" && date[1] == "31") || (date[0] == "DEC" && date[1] == "25") {
        println!("yup");
    } else {
        println!("nope");
    }
}

fn oddmanout() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|line| line.unwrap())
        .map(|line| line.parse().unwrap()).collect();
    let n: usize = lines[0].parse().unwrap();

    for i in (2..(2 * n + 1)).step_by(2) {
        let mut case: Vec<u32> = lines[i].split_whitespace().map(|s| s.trim())
            .map(|s| s.parse().unwrap())
            .collect();
        let mut alone = 0;
        for i in 0..case.len() {
            alone = alone ^ case[i];
        }
        println!("Case #{}: {}", i / 2, alone);
    }
}

fn datum() {
    let month_to_day: [usize; 12] = [3, 6, 6, 2, 4, 0, 2, 5, 1, 3, 6, 1];
    let day_to_output: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    let day: usize = input[0].parse().unwrap();
    let month: usize = input[1].parse().unwrap();
    println!("{}", day_to_output[(month_to_day[month - 1] + day) % 7]);
}

fn zamka() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|line| line.unwrap())
        .map(|line| line.parse().unwrap()).collect();
    let l: usize = lines[0].parse().unwrap();
    let d: usize = lines[1].parse().unwrap();
    let x: usize = lines[2].parse().unwrap();
}

fn aaah() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|line| line.unwrap())
        .map(|line| line.parse().unwrap()).collect();

    if lines[0].len() < lines[1].len() {
        println!("no");
    } else {
        println!("go")
    }
}

fn deathknight() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|line| line.unwrap())
        .map(|line| line.parse().unwrap()).collect();

    let mut result: u8 = lines[0].parse().unwrap();
    for i in 1..lines.len() {
        if lines[i].contains("CD") {
            result -= 1;
        }
    }
    println!("{}", result);
}

fn apaxiaaans() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input: Vec<char> = input.chars().collect();
    input.dedup();
    let input: String = input.iter().collect();
    println!("{}", input);
}

fn addingwords() {
    let unknown: String = String::from("unknown");
    let mut hashmap: HashMap<String, i16> = HashMap::with_capacity(32);
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let words: Vec<&str> = line.split_whitespace().collect();

        match words[0] {
            "def" => {
                hashmap.remove(words[1]);
                hashmap.insert(words[1].to_string(), words[2].parse().unwrap());
            }
            "calc" => {
                let mut has_unknown: bool = false;
                let mut value_total: i16 = match hashmap.get(words[1]) {
                    Some(v) => v.clone(),
                    None => {
                        has_unknown = true;
                        println!("{} {}", &line[5..line.len()], unknown);
                        0
                    }
                };

                if !has_unknown {
                    for i in (2..words.len() - 1).step_by(2) {
                        match hashmap.get(words[i + 1]) {
                            Some(v) => {
                                if words[i] == "+" {
                                    value_total += v;
                                } else {
                                    value_total -= v;
                                }
                            }
                            None => {
                                has_unknown = true;
                            }
                        };
                    }
                    if !has_unknown {
                        let key: String = match hashmap.iter()
                            .find(|&(_k, v)| v == &value_total) {
                            Some((k, &_v)) => k.clone(),
                            None => String::from("unknown")
                        };
                        println!("{} {}", &line[5..line.len()], key);
                    } else {
                        println!("{} {}", &line[5..line.len()], unknown);
                    }
                }
            }
            "clear" => hashmap.clear(),
            _ => return,
        }
    }
}

struct Quest {
    energy: u32,
    gold: u32,
}

struct Query {
    energy: u32,
    line: usize,
}

fn kattissquest() {
    let mut quests: Vec<Quest> = Vec::new();
    let mut queries: Vec<Query> = Vec::new();
    {
        let stdin = io::stdin();
        let mut i = 0;

        for line in stdin.lock().lines().map(|l| l.unwrap()) {
            let input: Vec<&str> = line.split_whitespace().collect();

            if input[0] == "add" {
                quests.push(Quest {
                    energy: input[1].parse().unwrap(),
                    gold: input[2].parse().unwrap(),
                });
            } else {
                queries.push(Query {
                    energy: input[1].parse().unwrap(),
                    line: i,
                });
            }
            i += 1;
        }
    }
    let mut completed_quests: usize = 0;
    let mut completed_queries: usize = 0;

    for query in queries.iter() {
        let range: usize = query.line - completed_quests - completed_queries;
        let mut achievable_quests: Vec<usize> = Vec::new();

        for i in 0..range {
            if quests[i].energy <= query.energy {
                achievable_quests.push(i);
            }
        }
        match achievable_quests.len() {
            0 => println!("{}", "0"),
            1 => {
                println!("{}", quests[achievable_quests[0]].gold);
                quests.remove(achievable_quests[0]);
                completed_quests += 1;
            }
            _ => {
                let mut total_gold: u32 = 0;
                let mut total_energy: u32 = 0;
                let mut accepted_quests: Vec<usize> = Vec::new();
                for _i in 0..achievable_quests.len() {
                    let mut largest_gold: usize = 0;
                    for i in 0..achievable_quests.len() {
                        if quests[achievable_quests[i]].gold >= quests[largest_gold].gold {
                            largest_gold = achievable_quests[i];
                        }
                    }
                    total_gold += quests[largest_gold].gold;
                    accepted_quests.push(largest_gold);
                    largest_gold = 0;
                }
                //Refactoring possible for copy-less order
                /*let mut ordered_quests: Vec<usize> = Vec::with_capacity(achievable_quests.len());
                {
                    let mut largest_gold: usize = 0;
                    for _i1 in 0..achievable_quests.len() {
                        for i2 in 0..achievable_quests.len() {
                            if quests[achievable_quests[i2]].gold >= quests[achievable_quests[largest_gold]].gold {
                                largest_gold = i2;
                            }
                        }
                        ordered_quests.push(achievable_quests[largest_gold]);
                        achievable_quests.remove(largest_gold);
                        largest_gold = 0;
                    }
                }*/
            }
        }
        completed_queries += 1;
    }
}

fn anthonyanddiablo() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<f32> = input.split_whitespace().map(|word| word.parse().unwrap())
        .collect();
    let available = input[1] / (PI * 2.0);
    let size = PI * (available.powf(2.0));

    if size >= input[0] {
        println!("Diablo is happy!");
    } else {
        println!("Need more materials!");
    }
}

fn abc() {
    let mut numbers = String::with_capacity(6);
    io::stdin().read_line(&mut numbers).unwrap();
    let mut numbers: Vec<u8> = numbers.split_whitespace().map(|word| word.parse().unwrap())
        .collect();
    numbers.sort_unstable();

    let mut order = String::with_capacity(4);
    io::stdin().read_line(&mut order).unwrap();
    let order: Vec<char> = order.chars().collect();

    match order[0] {
        'A' => print!("{} ", numbers[0]),
        'B' => print!("{} ", numbers[1]),
        'C' => print!("{} ", numbers[2]),
        _ => return,
    }
    match order[1] {
        'A' => print!("{} ", numbers[0]),
        'B' => print!("{} ", numbers[1]),
        'C' => print!("{} ", numbers[2]),
        _ => return,
    }
    match order[2] {
        'A' => print!("{}", numbers[0]),
        'B' => print!("{}", numbers[1]),
        'C' => print!("{}", numbers[2]),
        _ => return,
    }
}

fn main() {
    abc();
}