use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let moves: Vec<(&str, &str)> = input.lines().map(|l| l.split_once(" ").unwrap()).collect();

    let mut score = 0;

    let mut my_moves = HashMap::new();
    my_moves.insert("X", 1);
    my_moves.insert("Y", 2);
    my_moves.insert("Z", 3);

    for (opp, me) in &moves {
        match (*opp, *me) {
            ("A", "X") | ("B", "Y") | ("C", "Z") => {
                // draw
                score += 3 + my_moves[me];
            }
            ("A", "Y") | ("B", "Z") | ("C", "X") => {
                // win
                score += 6 + my_moves[me];
            }
            ("A", "Z") | ("B", "X") | ("C", "Y") => {
                // lose
                score += my_moves[me];
            }
            _ => {
                unreachable!()
            }
        }
    }

    println!("Part 1 = {}", score);

    score = 0;

    for (opp, me) in &moves {
        match (*opp, *me) {
            (_, "Y") => {
                // draw
                let my_move = match *opp {
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    _ => {
                        unreachable!()
                    }
                };
                score += 3 + my_move;
            }
            (_, "Z") => {
                // win
                let my_move = match *opp {
                    "A" => 2,
                    "B" => 3,
                    "C" => 1,
                    _ => {
                        unreachable!()
                    }
                };
                score += 6 + my_move;
            }
            (_, "X") => {
                // lose
                let my_move = match *opp {
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => {
                        unreachable!()
                    }
                };
                score += my_move;
            }
            _ => {
                unreachable!()
            }
        }
    }

    println!("Part 2 = {}", score);
}
