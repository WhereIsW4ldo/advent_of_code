use core::fmt;
use std::fs;
use regex::Regex;
extern crate queues;

pub fn day9() {
    let file_contents = read_file(false);

    // println!("{file_contents}");

    let lines = file_contents.split("\n").collect::<Vec<&str>>();

    // Regex setup:
    let am = Regex::new(r".\s(\d*)").unwrap();

    let mut rope: Rope = Rope::new();

    for line in lines
    {
        // println!("input: {line}");
        let mut line_char = line.chars();
        let dir = line_char.nth(0).unwrap();
        let mut amount: i32 = 0;
        for mat in am.captures_iter(line)
        {
            amount = mat[1].parse::<i32>().unwrap();
        }

        rope.move_all(dir, amount);
    }

    println!("pos: {}", rope.visited_places.len());
}

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    visited_places: Vec<(i32, i32)>,
}

impl Rope {
    pub fn new() -> Rope{
        Rope{ head: (0, 0), tail: (0, 0), visited_places: Vec::new() }
    }

    pub fn move_all(&mut self, direction: char, amount: i32) {
        for _ in 0..amount {
            self.move_head(direction);
        }
    }

    pub fn distance(&self) -> i32 {
        (self.head.0 - self.tail.0).abs() + (self.head.1 - self.tail.0).abs()
    }

    pub fn move_head(&mut self, direction: char) {
        match direction {
            'L' => {
                self.head.1 -= 1;
            },
            'R' => {
                self.head.1 += 1;
            },
            'D' => {
                self.head.0 -= 1;
            },
            'U' => {
                self.head.0 += 1;
            },
            _ => println!("this should not happen, wrong direction detected")
        }
        self.move_tail();
        // println!("{}", self);
    }

    pub fn move_tail(&mut self) {
        // println!("dist: {}", self.distance());

        if self.head.1 - self.tail.1 >= 2 {
            self.tail.1 = self.head.1 - 1;
            self.tail.0 = self.head.0;
        }
        if self.head.1 - self.tail.1 <= -2 {
            self.tail.1 = self.head.1 + 1;
            self.tail.0 = self.head.0;
        }
        if self.head.0 - self.tail.0 >= 2{
            self.tail.0 = self.head.0 - 1;
            self.tail.1 = self.head.1;
        }
        if self.head.0 - self.tail.0 <= -2 {
            self.tail.0 = self.head.0 + 1;
            self.tail.1 = self.head.1;
        }

        if !self.visited_places.contains((&self.tail)) {
            self.visited_places.push(self.tail);
        }
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "head: ({}, {}), tail: ({}, {})\ntrail: {:?}", self.head.0, self.head.1, self.tail.0, self.tail.1, self.visited_places)
    }
}

fn read_file(dummy: bool) -> String {
    match dummy {
        true => {
            String::from("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2")
        },
        false => {
            fs::read_to_string("day9.txt").unwrap()
        },
    }
}