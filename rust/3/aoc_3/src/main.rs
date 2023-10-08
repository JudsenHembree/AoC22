use std::collections::HashMap;

#[derive(Debug)]
struct Comparmtent {
    // hash map of char
    char_map: HashMap<char, u32>,
}

impl Comparmtent{
    fn new() -> Comparmtent {
        Comparmtent {
            char_map: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Box {
    // pair of compartments
    compartments: (Comparmtent, Comparmtent),
}

struct Group {
    // vector of hash maps
    boxes: (HashMap<char, u32>, HashMap<char, u32>, HashMap<char, u32>),
}

impl Group {
    fn from_boxes(box1: &Box, box2: &Box, box3: &Box) -> Group {
        let mut group = Group { boxes: (HashMap::new(), HashMap::new(), HashMap::new()) };
        for key in box1.compartments.0.char_map.keys() {
            if group.boxes.0.contains_key(key) {
                *group.boxes.0.entry(*key).or_insert(0) += 1;
            } else {
                *group.boxes.0.entry(*key).or_insert(0) = 1;
            }
        }
        for key in box1.compartments.1.char_map.keys() {
            if group.boxes.0.contains_key(key) {
                *group.boxes.0.entry(*key).or_insert(0) += 1;
            } else {
                *group.boxes.0.entry(*key).or_insert(0) = 1;
            }
        }

        for key in box2.compartments.0.char_map.keys() {
            if group.boxes.1.contains_key(key) {
                *group.boxes.1.entry(*key).or_insert(0) += 1;
            } else {
                *group.boxes.1.entry(*key).or_insert(0) = 1;
            }
        }
        for key in box2.compartments.1.char_map.keys() {
            if group.boxes.1.contains_key(key) {
                *group.boxes.1.entry(*key).or_insert(0) += 1;
            } else {
                *group.boxes.1.entry(*key).or_insert(0) = 1;
            }
        }

        for key in box3.compartments.0.char_map.keys() {
            if group.boxes.2.contains_key(key) {
                *group.boxes.2.entry(*key).or_insert(0) += 1;
            } else {
                *group.boxes.2.entry(*key).or_insert(0) = 1;
            }
        }
        for key in box3.compartments.1.char_map.keys() {
            if group.boxes.2.contains_key(key) {
                *group.boxes.2.entry(*key).or_insert(0) += 1;
            } else {
                *group.boxes.2.entry(*key).or_insert(0) = 1;
            }
        }
        group
    }

    fn discover_badge(&self) -> Result<char, &str>{
        let mut badge = ' ';
        for key in self.boxes.0.keys() {
            if self.boxes.1.contains_key(key) && self.boxes.2.contains_key(key) {
                badge = *key;
            }
        }
        if badge == ' ' {
            Err("No badge found")
        } else {
            Ok(badge)
        }
    }
}

fn main() {
    let mut boxes = Vec::new();
    // read the file
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    for line in input.lines() {
        // split the line into chars
        let mut chars = line.chars();
        // create a new compartment with the first half of the line
        let mut compartment = Comparmtent::new();
        for _ in 0..(line.len() / 2) {
            // get the next char
            let c = chars.next().unwrap();
            // add it to the compartment
            *compartment.char_map.entry(c).or_insert(0) += 1;
        }
        // create a new compartment with the second half of the line
        let mut compartment2 = Comparmtent::new();
        for _ in 0..(line.len() / 2) {
            // get the next char
            let c = chars.next().unwrap();
            // add it to the compartment
            *compartment2.char_map.entry(c).or_insert(0) += 1;
        }
        // create a new box with the two compartments
        let box_ = Box {
            compartments: (compartment, compartment2),
        };
        // add the box to the vector
        boxes.push(box_);
    }

    let mut items = Vec::new();
    for bo in &boxes {
        for key in bo.compartments.0.char_map.keys() {
            // check if the key is in the second compartment
            if bo.compartments.1.char_map.contains_key(key) {
                //copy the key into items
                items.push(*key);
            }
        }
    }

    let mut prio = 0;

    for item in items {
        // if lowercase
        if item.is_lowercase() {
            prio += item as u32 - 96;
        } else {
            // if uppercase
            prio += item as u32 - 38;
        }
    }

    println!("The priority is: {}", prio);

    // part 2
    prio = 0;
    // for every box step by 3
    for i in (0..boxes.len()).step_by(3) {
        // create a new group
        let group = Group::from_boxes(&boxes[i], &boxes[i + 1], &boxes[i + 2]);
        // get the badge
        let badge = group.discover_badge();
        match badge {
            Ok(b) => {
                // if lowercase
                if b.is_lowercase() {
                    prio += b as u32 - 96;
                } else {
                    // if uppercase
                    prio += b as u32 - 38;
                }
            }
            Err(_) => {}
        }
    }
    println!("The priority is: {}", prio);
}
