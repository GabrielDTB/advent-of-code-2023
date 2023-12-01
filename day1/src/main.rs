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
                l.replace("zero", "0")
                    .replace("one", "1")
                    .replace("two", "2")
                    .replace("three", "3")
                    .replace("four", "4")
                    .replace("five", "5")
                    .replace("six", "6")
                    .replace("seven", "7")
                    .replace("eight", "8")
                    .replace("nine", "9")
            })
            .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
            .map(|l| format!("{}{}", l.chars().next().unwrap(), l.chars().last().unwrap()))
            .map(|l| l.parse::<u32>().unwrap())
            .sum::<u32>()
    );
}
