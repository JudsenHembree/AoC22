// create a struct
struct Elf{
    // vector of int
    snacks: Vec<u32>,
    sum: u32,
}

impl Elf {
    fn new() -> Elf {
        Elf {
            snacks: Vec::new(),
            sum: 0,
        }
    }

    fn add_snack(&mut self, snack: u32) {
        self.snacks.push(snack);
    }

    fn sum_snacks(&self) -> u32 {
        let mut sum: u32 = 0;
        for snack in self.snacks.iter() {
            sum += snack;
        }
        sum
    }

    // impl copy
    fn copy(&self) -> Elf {
        Elf {
            snacks: self.snacks.clone(),
            sum: self.sum,
        }
    }
}

struct Elves{
    // vector of Elf
    elves: Vec<Elf>,
}

impl Elves{
    fn new() -> Elves {
        Elves {
            elves: Vec::new(),
        }
    }

    fn add_elf(&mut self, elf: Elf) {
        self.elves.push(elf);
    }

    fn sum_elves(&self) -> u32 {
        let mut sum: u32 = 0;
        for elf in self.elves.iter() {
            sum += elf.sum_snacks();
        }
        sum
    }

    fn sort_elves(&mut self) {
        self.elves.sort_by(|a, b| a.sum.cmp(&b.sum));
    }

}

fn main() {
    let mut elves = Elves::new();
    let mut max_calories: u32 = 0;
    let mut top_three = Elves::new();
    // read the input file 
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    // split input on new line
    let mut elf = Elf::new();
    for inp in input.lines() {
        // convert inp to string
        println!("The input is: {}", inp);
        match inp {
            "" => {
                elf.sum = elf.sum_snacks();
                if elf.sum > max_calories {
                    max_calories = elf.sum;
                }
                if top_three.elves.len() < 3 {
                    top_three.add_elf(elf.copy());
                } else {
                    top_three.sort_elves();
                    if elf.sum > top_three.elves[0].sum {
                        top_three.elves[0] = elf.copy();
                    }
                }
                elves.add_elf(elf);
                elf = Elf::new();
            },
            _ => {
                elf.add_snack(inp.parse::<u32>().unwrap());
            },
        }
    }
    println!("The total calories of all the elves is: {}", elves.sum_elves());
    println!("The maximum calories of all the elves is: {}", max_calories);
    println!("The top three elves are: {}", top_three.sum_elves());
}

