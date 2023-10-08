// use hashmap
use std::collections::HashMap;
// use deque
use std::collections::VecDeque;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
    visited: HashMap<(u32, u32), bool>,
    next_moves: VecDeque<(u32, u32)>
}

impl Map{
    fn from_input(input: &str) -> Map {
        let mut data = Vec::new();

        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            data.push(row);
        }

        let width = data[0].len();
        let height = data.len();

        let visited = HashMap::new();
        let next_moves = VecDeque::new();

        Map {
            width,
            height,
            data,
            visited,
            next_moves
        }
    }

    fn find_self(&self) -> Result<(u32, u32), &'static str> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.data[y as usize][x as usize] == 'S' {
                    return Ok((x as u32, y as u32));
                }
            }
        }
        Err("Could not find self")
    }

    fn process(&mut self) -> Result<u32, &'static str> {
        let mut steps = 0;
        let current_position = self.find_self()?;
        self.next_moves.push_back(current_position);
        let mut finished = false;
        while !finished {
            steps += 1;
            let mut moves = self.next_moves.len();
            while moves > 0 {
                let current_position = self.next_moves.pop_front().unwrap();
                let (x, y) = current_position;
                if self.data[y as usize][x as usize] == 'E' {
                    println!("Found exit!");
                    finished = true;
                    break;
                }
                if self.visited.contains_key(&current_position) {
                    moves -= 1;
                    continue;
                }
                self.visited.insert(current_position, true);
                if x > 0 {
                    let new_position = (x - 1, y);
                    let mut left = self.data[y as usize][(x - 1) as usize] as u32;
                    if left == 'E' as u32 {
                        left = 'z' as u32;
                    }
                    let mut right = self.data[y as usize][x as usize] as u32;
                    if right == 'S' as u32 {
                        right = 'a' as u32;
                    }
                    if left <=  right + 1 {
                        self.next_moves.push_back(new_position);
                    }
                }
                if x < (self.width - 1).try_into().unwrap() {
                    let new_position = (x + 1, y);
                    let mut left = self.data[y as usize][(x + 1) as usize] as u32;
                    if left == 'E' as u32 {
                        left = 'z' as u32;
                    }
                    let mut right = self.data[y as usize][x as usize] as u32;
                    if right == 'S' as u32 {
                        right = 'a' as u32;
                    }
                    if left <=  right + 1 {
                        self.next_moves.push_back(new_position);
                    }
                }
                if y > 0 {
                    let new_position = (x, y - 1);
                    let mut left = self.data[(y - 1) as usize][x as usize] as u32;
                    let mut right = self.data[y as usize][x as usize] as u32;
                    if left == 'E' as u32 {
                        left = 'z' as u32;
                    }
                    if right == 'S' as u32 {
                        right = 'a' as u32;
                    }
                    if left <=  right + 1 {
                        self.next_moves.push_back(new_position);
                    }
                }
                if y < (self.height - 1).try_into().unwrap() {
                    let new_position = (x, y + 1);
                    let mut left = self.data[(y + 1) as usize][x as usize] as u32;
                    let mut right = self.data[y as usize][x as usize] as u32;
                    if left == 'E' as u32 {
                        left = 'z' as u32;
                    }
                    if right == 'S' as u32 {
                        right = 'a' as u32;
                    }
                    if left <=  right + 1 {
                        self.next_moves.push_back(new_position);
                    }
                }
                moves -= 1;
            }
        }
        return Ok(steps - 1);
    }

    fn clean(&mut self) {
        self.visited.clear();
        self.next_moves.clear();
    }

    fn find_all_a(&self) -> Result<VecDeque<(u32, u32)>, &'static str> {
        let mut positions = VecDeque::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.data[y as usize][x as usize] == 'S' || self.data[y as usize][x as usize] == 'a' {
                    positions.push_back((x as u32, y as u32));
                }
            }
        }
        Ok(positions)
    }

    fn process_2(&mut self) -> Result<u32, &'static str> {
        let starts = self.find_all_a()?;
        let mut runs = Vec::new();
        for start in starts {
            self.clean();
            let mut steps = 0;
            self.next_moves.push_back(start);
            let mut finished = false;
            while !finished {
                let mut moves = self.next_moves.len();
                if moves == 0 {
                    break;
                }
                steps += 1;
                while moves > 0 {
                    let current_position = self.next_moves.pop_front().unwrap();
                    let (x, y) = current_position;
                    if self.data[y as usize][x as usize] == 'E' {
                        println!("Found exit!");
                        finished = true;
                        break;
                    }
                    if self.visited.contains_key(&current_position) {
                        moves -= 1;
                        continue;
                    }
                    self.visited.insert(current_position, true);
                    if x > 0 {
                        let new_position = (x - 1, y);
                        let mut left = self.data[y as usize][(x - 1) as usize] as u32;
                        if left == 'E' as u32 {
                            left = 'z' as u32;
                        }
                        let mut right = self.data[y as usize][x as usize] as u32;
                        if right == 'S' as u32 {
                            right = 'a' as u32;
                        }
                        if left <=  right + 1 {
                            self.next_moves.push_back(new_position);
                        }
                    }
                    if x < (self.width - 1).try_into().unwrap() {
                        let new_position = (x + 1, y);
                        let mut left = self.data[y as usize][(x + 1) as usize] as u32;
                        if left == 'E' as u32 {
                            left = 'z' as u32;
                        }
                        let mut right = self.data[y as usize][x as usize] as u32;
                        if right == 'S' as u32 {
                            right = 'a' as u32;
                        }
                        if left <=  right + 1 {
                            self.next_moves.push_back(new_position);
                        }
                    }
                    if y > 0 {
                        let new_position = (x, y - 1);
                        let mut left = self.data[(y - 1) as usize][x as usize] as u32;
                        let mut right = self.data[y as usize][x as usize] as u32;
                        if left == 'E' as u32 {
                            left = 'z' as u32;
                        }
                        if right == 'S' as u32 {
                            right = 'a' as u32;
                        }
                        if left <=  right + 1 {
                            self.next_moves.push_back(new_position);
                        }
                    }
                    if y < (self.height - 1).try_into().unwrap() {
                        let new_position = (x, y + 1);
                        let mut left = self.data[(y + 1) as usize][x as usize] as u32;
                        let mut right = self.data[y as usize][x as usize] as u32;
                        if left == 'E' as u32 {
                            left = 'z' as u32;
                        }
                        if right == 'S' as u32 {
                            right = 'a' as u32;
                        }
                        if left <=  right + 1 {
                            self.next_moves.push_back(new_position);
                        }
                    }
                    moves -= 1;
                }
            }
            if finished {
                runs.push(steps - 1);
            }
        }
        // sort the runs
        runs.sort();
        return Ok(runs[0]);
    }
}

fn main() {
    // read the input file
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let mut map = Map::from_input(&input);
    let res = map.process();
    println!("Steps: {:?}", res);
    map.clean();
    let res_2 = map.process_2();
    println!("Steps: {:?}", res_2);
}
