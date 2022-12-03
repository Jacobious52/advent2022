fn main() {
    let input = include_str!("../input.txt");

    let mut elves: Vec<i32> = input
        .split("\n\n")
        .map(|f| f.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .collect();

    elves.sort();

    println!("Part 1 = {}", elves.last().unwrap());
    println!("Part 2 = {}", elves[elves.len() - 3..].iter().sum::<i32>());
}
