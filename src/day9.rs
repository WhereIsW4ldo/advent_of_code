use regex::Regex;
use std::{fmt, fs, process::exit};
extern crate queues;

pub fn day9() {
    let file_contents = read_file(false);

    // println!("{file_contents}");

    let lines = file_contents.split("\n").collect::<Vec<&str>>();

    // Regex setup:
    let am = Regex::new(r".\s(\d*)").unwrap();

    let mut rope: Rope = Rope::new();

    for (i, line) in lines.iter().enumerate() {
        // println!("input: {line}");
        let mut line_char = line.chars();
        let dir = line_char.nth(0).unwrap();
        let mut amount: i32 = 0;
        for mat in am.captures_iter(line) {
            amount = mat[1].parse::<i32>().unwrap();
        }

        rope.move_all(dir, amount);

        // if i == 10 {
        //     return;
        // }
    }

    println!("{}", rope);

    println!("pos: {}", rope.visited_places.len());
}

// struct Rope {
//     head: (i32, i32),
//     tail: (i32, i32),
//     visited_places: Vec<(i32, i32)>,
// }

// impl Rope {
//     pub fn new() -> Rope{
//         Rope{ head: (0, 0), tail: (0, 0), visited_places: Vec::new() }
//     }

//     pub fn move_all(&mut self, direction: char, amount: i32) {
//         for _ in 0..amount {
//             self.move_head(direction);
//         }
//     }

//     pub fn distance(&self) -> i32 {
//         (self.head.0 - self.tail.0).abs() + (self.head.1 - self.tail.0).abs()
//     }

//     pub fn move_head(&mut self, direction: char) {
//         match direction {
//             'L' => {
//                 self.head.1 -= 1;
//             },
//             'R' => {
//                 self.head.1 += 1;
//             },
//             'D' => {
//                 self.head.0 -= 1;
//             },
//             'U' => {
//                 self.head.0 += 1;
//             },
//             _ => println!("this should not happen, wrong direction detected")
//         }
//         self.move_tail();
//         // println!("{}", self);
//     }

//     pub fn move_tail(&mut self) {
//         // println!("dist: {}", self.distance());

//         if self.head.1 - self.tail.1 >= 2 {
//             self.tail.1 = self.head.1 - 1;
//             self.tail.0 = self.head.0;
//         }
//         if self.head.1 - self.tail.1 <= -2 {
//             self.tail.1 = self.head.1 + 1;
//             self.tail.0 = self.head.0;
//         }
//         if self.head.0 - self.tail.0 >= 2{
//             self.tail.0 = self.head.0 - 1;
//             self.tail.1 = self.head.1;
//         }
//         if self.head.0 - self.tail.0 <= -2 {
//             self.tail.0 = self.head.0 + 1;
//             self.tail.1 = self.head.1;
//         }

//         if !self.visited_places.contains((&self.tail)) {
//             self.visited_places.push(self.tail);
//         }
//     }
// }

// impl fmt::Display for Rope {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "head: ({}, {}), tail: ({}, {})\ntrail: {:?}", self.head.0, self.head.1, self.tail.0, self.tail.1, self.visited_places)
//     }
// }

struct Rope {
    head: (i32, i32),
    tail: Vec<(i32, i32)>,
    visited_places: Vec<(i32, i32)>,
}

impl Rope {
    pub fn new() -> Rope {
        let mut queue: Vec<(i32, i32)> = Vec::new();
        for _ in 0..9 {
            queue.push((0, 0));
        }
        Rope {
            head: (0, 0),
            tail: queue,
            visited_places: Vec::new(),
        }
    }

    pub fn move_all(&mut self, direction: char, amount: i32) {
        for _ in 0..amount {
            self.move_head(direction);
        }
    }

    pub fn move_head(&mut self, direction: char) {

        match direction {
            'L' => {
                self.head.1 -= 1;
            }
            'R' => {
                self.head.1 += 1;
            }
            'D' => {
                self.head.0 -= 1;
            }
            'U' => {
                self.head.0 += 1;
            }
            _ => println!("this should not happen, wrong direction detected"),
        }
        // println!("head: {},{}", self.head.0, self.head.1);

        self.move_tail();

    }

    pub fn move_tail_item(&mut self, index: usize) {
        if self.tail.get(index).unwrap().1 - self.tail.get(index - 1).unwrap().1 >= 2 {
            self.tail.get_mut(index - 1).unwrap().0 = self.tail.get(index).unwrap().0;
            self.tail.get_mut(index - 1).unwrap().1 = self.tail.get(index).unwrap().1 - 1;
            // self.tail.push((self.head.0, self.head.1 - 1));
        }
        if self.tail.get(index).unwrap().1 - self.tail.get(index - 1).unwrap().1 <= -2 {
            self.tail.get_mut(index - 1).unwrap().0 = self.tail.get(index).unwrap().0;
            self.tail.get_mut(index - 1).unwrap().1 = self.tail.get(index).unwrap().1 + 1;
            // self.tail.push((self.head.0, self.head.1 + 1));
        }
        if self.tail.get(index).unwrap().0 - self.tail.get(index - 1).unwrap().0 >= 2 {
            self.tail.get_mut(index - 1).unwrap().0 = self.tail.get(index).unwrap().0 - 1;
            self.tail.get_mut(index - 1).unwrap().1 = self.tail.get(index).unwrap().1;
            // self.tail.push((self.head.0 - 1, self.head.1));
        }
        if self.tail.get(index).unwrap().0 - self.tail.get(index - 1).unwrap().0 <= -2 {
            self.tail.get_mut(index - 1).unwrap().0 = self.tail.get(index).unwrap().0 + 1;
            self.tail.get_mut(index - 1).unwrap().1 = self.tail.get(index).unwrap().1;
            // self.tail.push((self.head.0, self.head.1 + 1));
        }
    }

    pub fn move_tail(&mut self) {
        // println!("tail: {:?}", &self.tail);
        let mut add = false;

        if self.head.1 - self.tail.last().unwrap().1 >= 2 {
            self.tail.last_mut().unwrap().0 = self.head.0;
            self.tail.last_mut().unwrap().1 = self.head.1 - 1;
            // self.tail.push((self.head.0, self.head.1 - 1));
            // self.tail.1 = self.head.1 - 1;
            // self.tail.0 = self.head.0;
            add = true;
        }
        if self.head.1 - self.tail.last().unwrap().1 <= -2 {
            self.tail.last_mut().unwrap().0 = self.head.0;
            self.tail.last_mut().unwrap().1 = self.head.1 + 1;
            // self.tail.push((self.head.0, self.head.1 + 1));
            add = true;
        }
        if self.head.0 - self.tail.last().unwrap().0 >= 2 {
            self.tail.last_mut().unwrap().0 = self.head.0 - 1;
            self.tail.last_mut().unwrap().1 = self.head.1;
            // self.tail.push((self.head.0 - 1, self.head.1));
            add = true;
        }
        if self.head.0 - self.tail.last().unwrap().0 <= -2 {
            self.tail.last_mut().unwrap().0 = self.head.0 + 1;
            self.tail.last_mut().unwrap().1 = self.head.1;
            // self.tail.push((self.head.0, self.head.1 + 1));
            add = true;
        }

        

        for index in (1..9).rev() {
            // println!("index: {index}");
            self.move_tail_item(index);
        }
        if add {
            // add last just removed element to visited_places
            // let t = self.tail.remove(0);
            let temp = self.tail.get(0).unwrap();
            if !self.visited_places.contains(temp) {
                self.visited_places.push(*temp);
                // println!("places: {:?}", &self.visited_places);
            }
        }
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("head: {},{}", self.head.0, self.head.1);
        println!("tail: {:?}", self.tail);
        // 15 y+, -5 y-, 14 x+, -11 x-
        let mut temp = String::new();
        for y in (-5..15).rev() {
            for x in -11..14 {
                // if self.head.0 == y && self.head.1 == x {
                //     temp += "H";
                // } else if !self.tail.contains(&(y, x)) {
                //     temp += ".";
                // } else {
                //     for (i, t) in self.tail.iter().enumerate() {
                //         if t.0 == y && t.1 == x {
                //             temp += &i.to_string();
                //             break;
                //         }
                //     }
                // }
                if self.visited_places.contains(&(y, x)) {
                    temp += "#";
                }
                else {
                    temp += ".";
                }
            }
            temp += "\n";
        }
        write!(f, "{}\n\n", temp)
    }
}

fn read_file(dummy: bool) -> String {
    match dummy {
        true => String::from(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        ),
        false => fs::read_to_string("day9.txt").unwrap(),
    }
}
