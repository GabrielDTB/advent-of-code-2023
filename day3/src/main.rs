fn main() {
    part1();
    part2();
}

fn part1() {
    let input = String::from_utf8(std::fs::read("input").unwrap())
        .unwrap()
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let ylen = input.len() as i64;
    let xlen = input[0].len() as i64;
    let mut numbers = vec![];
    let mut buffer = Vec::<char>::with_capacity(4);
    let mut valid = false;
    for i in 0..ylen {
        if valid {
            numbers.push(buffer.iter().collect::<String>());
            valid = false;
        }
        if !buffer.is_empty() {
            buffer.clear();
        }
        for j in 0..xlen {
            match input[i as usize][j as usize] {
                c @ '0'..='9' => buffer.push(c),
                _ => {
                    if valid {
                        numbers.push(buffer.iter().collect::<String>());
                        valid = false;
                    }
                    if !buffer.is_empty() {
                        buffer.clear();
                    }
                    continue;
                }
            };
            for y in -1..=1 {
                if i + y < 0 || i + y >= ylen {
                    continue;
                }
                for x in -1..=1 {
                    if j + x < 0 || j + x >= xlen {
                        continue;
                    }
                    match input[(i + y) as usize][(j + x) as usize] {
                        '.' | '0'..='9' => continue,
                        _ => valid = true,
                    };
                }
            }
        }
    }
    if valid {
        numbers.push(buffer.iter().collect::<String>());
    }
    println!(
        "{:#?}",
        numbers
            .iter()
            .map(|n| n.parse::<usize>().unwrap())
            .sum::<usize>()
    );
}

fn part2() {
    let input = String::from_utf8(std::fs::read("input").unwrap())
        .unwrap()
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let ylen = input.len() as i64;
    let xlen = input[0].len() as i64;
    let mut numbers = vec![];
    let mut buffer = Vec::<char>::with_capacity(4);
    let mut valid = 0;
    for i in 0..ylen {
        if valid == 2 {
            numbers.push(buffer.iter().collect::<String>());
            valid = 0;
        }
        if !buffer.is_empty() {
            buffer.clear();
        }
        for j in 0..xlen {
            match input[i as usize][j as usize] {
                c @ '0'..='9' => buffer.push(c),
                _ => {
                    if valid {
                        numbers.push(buffer.iter().collect::<String>());
                        valid = false;
                    }
                    if !buffer.is_empty() {
                        buffer.clear();
                    }
                    continue;
                }
            };
            for y in -1..=1 {
                if i + y < 0 || i + y >= ylen {
                    continue;
                }
                for x in -1..=1 {
                    if j + x < 0 || j + x >= xlen {
                        continue;
                    }
                    match input[(i + y) as usize][(j + x) as usize] {
                        '.' | '0'..='9' => continue,
                        _ => valid = true,
                    };
                }
            }
        }
    }
    if valid {
        numbers.push(buffer.iter().collect::<String>());
    }
    println!(
        "{:#?}",
        numbers
            .iter()
            .map(|n| n.parse::<usize>().unwrap())
            .sum::<usize>()
    );
}
