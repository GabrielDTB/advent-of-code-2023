use std::cmp::max;

fn main() {
    task_one();
    task_two();
}

fn task_one() {
    let mut index = 1;
    println!(
        "{}",
        String::from_utf8(std::fs::read("input").unwrap())
            .unwrap()
            .split("\n")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.split_once(":").unwrap().1)
            .map(|l| l
                .split(";")
                .map(|c| c.split(",").map(|d| d.trim().split(" "))))
            .map(|l| l.flatten())
            .filter_map(|l| {
                let mut valid = Some(index);
                for mut d in l {
                    let count = d.next().unwrap().parse::<usize>().unwrap();
                    if count
                        > match d.next() {
                            Some("red") => 12,
                            Some("green") => 13,
                            Some("blue") => 14,
                            _ => panic!(),
                        }
                    {
                        valid = None;
                    }
                }
                index += 1;
                valid
            })
            .sum::<usize>()
    );
}

fn task_two() {
    println!(
        "{}",
        String::from_utf8(std::fs::read("input").unwrap())
            .unwrap()
            .split("\n")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.split_once(":").unwrap().1)
            .map(|l| l
                .split(";")
                .map(|c| c.split(",").map(|d| d.trim().split(" "))))
            .map(|l| l.flatten())
            .map(|l| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                for mut d in l {
                    let count = d.next().unwrap().parse::<usize>().unwrap();
                    match d.next() {
                        Some("red") => red = max(red, count),
                        Some("green") => green = max(green, count),
                        Some("blue") => blue = max(blue, count),
                        _ => panic!(),
                    }
                }
                red * blue * green
            })
            .sum::<usize>()
    );
}
