use regex::Regex;
use std::fs;

pub fn day10() {
    let input = read_input(false);

    let input_vec = input.split("\n").collect::<Vec<&str>>();
    let addx = Regex::new(r"addx\s(.\d*)").unwrap();

    let mut count: Vec<(i32, i32)> = Vec::new();

    let mut cycle: i32 = 1;

    let mut x = 1;

    let mut sprite_position: i32 = x;

    for (i, line) in input_vec.iter().enumerate() {
        // println!("voor: {}: x: {x}", cycle);
        if ((cycle%40 - sprite_position%40)).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        if line.contains("noop") {
            cycle += 1;
        } else if addx.is_match(line) {
            let mat = addx.captures(line).unwrap();
            count.push((mat[1].parse::<i32>().unwrap(), 0));

            cycle += 1;

            if cycle % 40 == 1 {
                println!("");
            }
            increase_count(&mut count, &mut x);
            sprite_position = x+1;
            count.retain(|&c| c.1 < 2);

            if ((cycle%40 - sprite_position%40)).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
            cycle += 1;
        }

        if cycle % 40 == 1 {
            println!("");
        }

        increase_count(&mut count, &mut x);
        sprite_position = x+1;
        count.retain(|&c| c.1 < 2);
    }
}

fn increase_count(count: &mut Vec<(i32, i32)>, x: &mut i32) {
    for (amount, c) in count {
        *c += 1;
        if c == &2 {
            *x += *amount;
        }
    }
}

fn read_input(dummy: bool) -> String {
    match dummy {
        true => String::from(
            "addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop",
        ),
        false => String::from(fs::read_to_string("day10.txt").unwrap()),
    }
}
