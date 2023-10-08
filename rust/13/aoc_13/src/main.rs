
//derives
#[derive(Debug, Clone)]
struct BytePair {
    first: String,
    second: String,
}

impl BytePair {
    fn new(first: String, second: String) -> BytePair {
        BytePair {
            first,
            second,
        }
    }
}

fn process(file: &str) -> bool{
    let input = std::fs::read_to_string(file).unwrap();
    let lines = input.lines();
    let mut step = 0;
    let mut byte_pairs: Vec<BytePair> = Vec::new();
    let mut pair = BytePair::new(String::new(), String::new());
    for line in lines {
        step += 1;
        let ln = line.clone();
        if step == 3 {
            step = 0;
            continue;
        }
        if step == 1 {
            pair.first = ln.to_string();
        }
        if step == 2 {
            pair.second = ln.to_string();
            byte_pairs.push(pair.clone());
            pair.first = String::new();
            pair.second = String::new();
        }
    }

    for pair in byte_pairs {
        println!("{:?}", pair);
    }
    true
}

fn main() {
    // read input file
    process("input/input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(process("input/test.txt"));
    }
}
