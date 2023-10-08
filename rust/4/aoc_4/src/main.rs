#[derive(Debug)]
struct Pair {
    elf1: (u32, u32),
    elf2: (u32, u32)
}

impl Pair {
    fn new(elf1: (u32, u32), elf2: (u32, u32)) -> Pair {
        Pair { elf1, elf2 }
    }
}

fn main() {
    // read the input
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let mut overlaps = 0;
    let mut any_overlap: i32 = 0;
    for line in input.lines() {
        // split on the comma
        let split: Vec<_> = line.split(",").collect();
        if split.len() != 2 {
            panic!("Invalid input");
        }
        let first_nums = split[0].split("-").collect::<Vec<_>>();
        let second_nums = split[1].split("-").collect::<Vec<_>>();
        let pair = Pair::new((first_nums[0].parse::<u32>().unwrap(), first_nums[1].parse::<u32>().unwrap()),
            (second_nums[0].parse::<u32>().unwrap(), second_nums[1].parse::<u32>().unwrap()));
        if (pair.elf1.0 >= pair.elf2.0 && pair.elf1.1 <= pair.elf2.1) || 
            (pair.elf2.0 >= pair.elf1.0 && pair.elf2.1 <= pair.elf1.1) {
            overlaps += 1;
        }

        // if any overlap
        if (pair.elf1.0 >= pair.elf2.0 && pair.elf1.0 <= pair.elf2.1) || 
            (pair.elf2.0 >= pair.elf1.0 && pair.elf2.0 <= pair.elf1.1) {
            any_overlap += 1;
        }
    }
    println!("{} full overlaps", overlaps);
    println!("{} any overlaps", any_overlap);
}
