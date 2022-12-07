use std::collections::HashSet;

fn start_of_packet(n: usize, stream: &[(usize, char)]) -> usize {
    stream
        .windows(n)
        .find(|data| {
            let set = data
                .iter()
                .map(|(_, c)| c.to_owned())
                .collect::<HashSet<char>>();

            set.len() == n
        })
        .unwrap()
        .last()
        .unwrap()
        .0
        + 1
}

fn main() {
    let input = include_str!("../input.txt");

    let stream = input.char_indices().collect::<Vec<(usize, char)>>();

    println!("Part1 = {:?}", start_of_packet(4, &stream));
    println!("Part2 = {:?}", start_of_packet(14, &stream));
}
