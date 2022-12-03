use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let alphabet = ('a'..='z').chain('A'..='Z').into_iter().collect::<Vec<_>>();

    let sacks: Vec<(&str, &str)> = input.lines().map(|l| l.split_at(l.len() / 2)).collect();

    let total_part_a = sacks
        .iter()
        .map(|(a, b)| {
            let set_a = a.chars().collect::<HashSet<_>>();
            let set_b = b.chars().collect::<HashSet<_>>();

            let intersect = set_a.intersection(&set_b).next().unwrap();

            let priority = alphabet.iter().position(|c| *c == *intersect).unwrap() + 1;
            return priority;
        })
        .sum::<usize>();

    println!("Part 1 = {:?}", total_part_a);

    let sacks: Vec<&str> = input.lines().collect();
    let total_part_b = sacks
        .chunks(3)
        .map(|chunk| {
            let set_a = chunk[0].chars().collect::<HashSet<_>>();
            let set_b = chunk[1].chars().collect::<HashSet<_>>();
            let set_c = chunk[2].chars().collect::<HashSet<_>>();

            let intersect_inner = set_b
                .intersection(&set_c)
                .map(ToOwned::to_owned)
                .collect::<HashSet<_>>();

            let intersect = set_a.intersection(&intersect_inner).next().unwrap();

            let priority = alphabet.iter().position(|c| *c == *intersect).unwrap() + 1;

            return priority;
        })
        .sum::<usize>();

    println!("Part 2 = {:?}", total_part_b);
}
