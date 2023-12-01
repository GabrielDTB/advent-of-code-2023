fn main() {
    println!(
        "{}",
        String::from_utf8_lossy(&std::fs::read("input").unwrap())
            .parse::<String>()
            .unwrap()
            .split("\n")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.replace("zero", "z0o")
                    .replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e")
            })
            .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
            .map(|l| format!("{}{}", l.chars().next().unwrap(), l.chars().last().unwrap()))
            .map(|l| l.parse::<u32>().unwrap())
            .sum::<u32>()
    );
}
