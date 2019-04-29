#![allow(dead_code)]
#![feature(core_intrinsics)]

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

fn zamka_sum(mut number: u16) -> u16 {
    let mut sum = 0;

    while number > 0 {
        sum += number % 10;
        number /= 10;
    }
    sum
}

fn zamka() {
    let stdin = io::stdin();
    let input: Vec<u16> = stdin.lock().lines().map(|line| line.unwrap())
        .map(|line| line.parse().unwrap()).collect();

    let mut i = input[0];
    while i <= input[1] {
        if zamka_sum(i) == input[2] {
            println!("{}", i);
            break;
        }
        i += 1;
    }

    let mut i = input[1];
    while i >= input[0] {
        if zamka_sum(i) == input[2] {
            println!("{}", i);
            break;
        }
        i -= 1;
    }
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

fn addingwords() {
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
                        println!("{} unknown", &line[5..line.len()]);
                        0
                    }
                };

                if !has_unknown {
                    'a: for i in (2..words.len() - 1).step_by(2) {
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
                                break 'a;
                            }
                        };
                    }
                    if !has_unknown {
                        println!("{} {}", &line[5..line.len()], match hashmap.iter()
                            .find(|&(_k, v)| v == &value_total) {
                            Some((k, &_v)) => k.clone(),
                            None => String::from("unknown"),
                        });
                    } else {
                        println!("{} unknown", &line[5..line.len()]);
                    }
                }
            }
            "clear" => hashmap.clear(),
            _ => return,
        }
    }
}

fn election2() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|line| line.unwrap())
        .map(|line| line.parse().unwrap()).collect();

    let mut candidates: Vec<String> = Vec::with_capacity(20);
    let mut parties: Vec<String> = Vec::with_capacity(20);
    let mut voices: Vec<u16> = Vec::with_capacity(20);
    {
        let n: usize = lines[0].parse().unwrap();

        for i in (1..n * 2 + 1).step_by(2) {
            candidates.push(lines[i].clone());
            parties.push(lines[i + 1].clone());
            voices.push(0);
        }
        for line in &lines[n * 2 + 2..lines.len()] {
            for (i, candidate) in candidates.iter().enumerate() {
                if candidate == line {
                    voices[i] += 1;
                    break;
                }
            }
        }
    }
    let mut i: usize = 1;
    let mut winner: usize = 0;
    let mut is_tie: bool = false;
    for voice in &voices[1..voices.len()] {
        let winner_voices = &voices[winner];

        if voice > winner_voices {
            winner = i;
            is_tie = false;
        } else if voice == winner_voices {
            is_tie = true;
        }
        i += 1;
    }
    if !is_tie {
        println!("{}", parties[winner]);
    } else {
        println!("tie");
    }
}

fn sibice() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|line| line.unwrap())
        .map(|line| line.parse().unwrap()).collect();
    let first_line: Vec<u16> = lines[0].split_whitespace()
        .map(|word| word.parse().unwrap()).collect();
    let dimension: f32 = ((first_line[1] * first_line[1] + first_line[2] * first_line[2]) as f32).sqrt();
    for line in &lines[1..lines.len()] {
        let n: f32 = line.parse().unwrap();

        if dimension >= n {
            println!("DA");
        } else {
            println!("NE");
        }
    }
}

fn modulo() {
    let stdin = io::stdin();
    let mut modulos: Vec<u16> = Vec::with_capacity(10);

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let number: u16 = line.parse().unwrap();
        modulos.push(number % 42);
    }
    modulos.sort();
    modulos.dedup();
    println!("{}", modulos.len());
}

const OUTPUT: [&str; 6] = ["Province", "Duchy", "Estate", "Gold", "Silver", "Copper"];

fn provincesandgold() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<u8> = input.split_whitespace()
        .map(|word| word.parse().unwrap()).collect();
    let total: u8 = input[0] * 3 + input[1] * 2 + input[2];
    let mut treasure: String = String::with_capacity(6);

    if total > 5 {
        treasure.push_str(OUTPUT[3]);
    } else if total > 2 {
        treasure.push_str(OUTPUT[4]);
    } else {
        treasure.push_str(OUTPUT[5]);
    }

    if total > 1 {
        let mut victory: String = String::with_capacity(8);

        if total > 7 {
            victory.push_str(OUTPUT[0]);
        } else if total > 4 {
            victory.push_str(OUTPUT[1]);
        } else {
            victory.push_str(OUTPUT[2]);
        }
        println!("{} or {}", victory, treasure);
    } else {
        println!("{}", treasure);
    }
}

fn pot() {
    let stdin = io::stdin();
    let stdin: Vec<u128> = stdin.lock().lines()
        .map(|l| l.unwrap().parse::<u128>().unwrap()).collect();
    let mut result: u128 = 0;

    for line in &stdin[1..stdin.len()] {
        result += (line / 10).pow((line % 10) as u32);
    }
    println!("{}", result);
}

fn planina() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim_end().parse().unwrap();
    println!("{}", (u64::pow(2, input) + 1).pow(2));
}

fn everywhere() {
    let stdin = io::stdin();
    let stdin: Vec<String> = stdin.lock().lines().map(|line| line.unwrap())
        .map(|line| line.parse().unwrap()).collect();
    let mut cities: Vec<&str> = Vec::with_capacity(100);
    let mut start: usize = 2;
    let mut end: usize = 1;

    for _i in 0..stdin[0].parse::<usize>().unwrap() {
        end = stdin[end].parse::<usize>().unwrap() + start;
        for line in &stdin[start..end] {
            if !cities.contains(&line.as_str()) {
                cities.push(line);
            }
        }
        println!("{}", cities.len());
        cities.clear();
        start = end + 1;
    }
}

fn detaileddifferences() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|line| line.unwrap())
        .map(|line| line.parse().unwrap()).collect();
    let mut result = String::with_capacity(50);
    let n: usize = lines[0].parse().unwrap();

    for i in (1..n * 2 + 1).step_by(2) {
        let first = lines[i].as_bytes();
        let second = lines[i + 1].as_bytes();

        for (i, &byte) in first.iter().enumerate() {
            if byte == second[i] {
                result.push('.');
            } else {
                result.push('*');
            }
        }
        println!("{}", lines[i]);
        println!("{}", lines[i + 1]);
        println!("{}", result);
        println!();
        result.clear();
    }
}

fn areal() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: f64 = input.trim_end().parse().unwrap();

    println!("{}", input.sqrt() * 4.0);
}

fn anewalphabet() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: String = input.parse().unwrap();
    let mut output: String = String::with_capacity(input.len() * 2);

    for c in input.to_ascii_lowercase().chars() {
        match c {
            'a' => {
                output.push('@');
            }
            'b' => {
                output.push('8');
            }
            'c' => {
                output.push('(');
            }
            'd' => {
                output.push_str("|)");
            }
            'e' => {
                output.push('3');
            }
            'f' => {
                output.push('#');
            }
            'g' => {
                output.push('6');
            }
            'h' => {
                output.push_str("[-]");
            }
            'i' => {
                output.push('|');
            }
            'j' => {
                output.push_str("_|");
            }
            'k' => {
                output.push_str("|<");
            }
            'l' => {
                output.push('1');
            }
            'm' => {
                output.push_str("[]\\/[]");
            }
            'n' => {
                output.push_str("[]\\[]");
            }
            'o' => {
                output.push('0');
            }
            'p' => {
                output.push_str("|D");
            }
            'q' => {
                output.push_str("(,)");
            }
            'r' => {
                output.push_str("|Z");
            }
            's' => {
                output.push('$');
            }
            't' => {
                output.push_str("']['");
            }
            'u' => {
                output.push_str("|_|");
            }
            'v' => {
                output.push_str("\\/");
            }
            'w' => {
                output.push_str("\\/\\/");
            }
            'x' => {
                output.push_str("}{");
            }
            'y' => {
                output.push_str("`/");
            }
            'z' => {
                output.push('2');
            }
            _ => {
                output.push(c);
            }
        };
    }

    println!("{}", output);
}

fn grassseed() {
    let stdin = io::stdin();
    let stdin: Vec<String> = stdin.lock().lines().map(|line| line.unwrap())
        .map(|line| line.parse().unwrap()).collect();
    let cost: f64 = stdin[0].parse().unwrap();
    let mut total: f64 = 0.0;
    for line in &stdin[2..stdin.len()] {
        let s: Vec<&str> = line.split_whitespace().collect();
        total += s[0].parse::<f64>().unwrap() * s[1].parse::<f64>().unwrap();
    }
    println!("{}", cost * total);
}

fn pet() {
    let stdin = io::stdin();
    let mut c: usize = 0;
    let mut hg: u8 = 0;
    for (i, line) in stdin.lock().lines().map(|l| l.unwrap()).enumerate() {
        let numbers: Vec<u8> = line.split_whitespace().map(|l| l.parse::<u8>().unwrap()).collect();
        let g = numbers[0] + numbers[1] + numbers[2] + numbers[3];
        if g > hg {
            hg = g;
            c = i;
        }
    }
    println!("{} {}", c + 1, hg);
}

fn bing() {
    let stdin = io::stdin();
    let stdin: Vec<String> = stdin.lock().lines().map(|line| line.unwrap())
        .collect();
    let mut i: usize = 1;

    for line in &stdin[1..stdin.len()] {
        let mut n: usize = 0;
        for word in &stdin[1..i] {
            if word.starts_with(line) {
                n += 1;
            }
        }
        println!("{}", n);
        i += 1;
    }
}

fn romans() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: f32 = input.trim_end().parse().unwrap();
    println!("{}", (input * (5280. / 4854.) * 1000. + 0.5) as u32);
}

fn ladder() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<f32> = input.split_whitespace().map(|n| n.parse::<f32>().unwrap()).collect();
    let r: f32 = input[1] * (PI / 180.);
    let r: f32 = input[0] / r.sin();
    println!("{}", r.ceil());
}

fn main() {
    ladder();
}