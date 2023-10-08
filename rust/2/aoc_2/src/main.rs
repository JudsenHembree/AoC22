#[derive(Debug)]
struct Round {
    theirs: char,
    ours: char,
}

impl Round {
    fn new(theirs: char, ours: char) -> Round {
        Round {
            theirs,
            ours,
        }
    }

    fn new_from_line(line: &str) -> Round {
        let chars = line.split(" ").collect::<Vec<&str>>();
        Round::new(chars[0].chars().next().unwrap(), chars[1].chars().next().unwrap())
    }
}
fn main() {
    let mut points: i32 = 0;
    // read the file
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let mut rounds: Vec<Round> = Vec::new();
    for line in input.lines() {
        rounds.push(Round::new_from_line(line));
    }
    for rd in rounds {
        match rd.theirs {
            'A' => { // rock
                match rd.ours {
                    'X' => {
                        // always lose
                        points += 0;
                        // tool
                        points += 3;
                    }
                    'Y' => {
                        // tie
                        points += 3;
                        //tool
                        points += 1;
                    }
                    'Z' => {
                        // win
                        points += 6;
                        //tool
                        points += 2;
                    }
                    _ => {}
                }
            }
            'B' => { // paper
                match rd.ours {
                    'X' => {
                        // always lose

                        // tool
                        points += 1;
                    }
                    'Y' => {
                        // tie
                        points += 3;

                        //tool
                        points += 2;
                    }
                    'Z' => {
                        // win
                        points += 6;

                        //tool
                        points += 3;
                    }
                    _ => {}
                }
            }
            'C' => { // scissors
                match rd.ours {
                    'X' => {
                        // always lose

                        // tool
                        points += 2;
                    }
                    'Y' => {
                        // tie
                        points += 3;

                        // tool
                        points += 3;
                    }
                    'Z' => {
                        // win
                        points += 6;

                        // tool
                        points += 1;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    println!("Points: {}", points);
}
