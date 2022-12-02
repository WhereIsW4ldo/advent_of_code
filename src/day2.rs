use core::fmt;
use std::{fs, process::exit};

#[derive(PartialEq, Copy, Clone)]
enum Hands
{
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for Hands
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hands::Rock => write!(f, "Rock"),
            Hands::Paper => write!(f, "Paper"),
            Hands::Scissors => write!(f, "Scissors"),
        }
    }
}

pub fn day2(){
    // opponent: A -> Rock, B -> Paper, C -> Scissors
    // you:      X -> Rock, Y -> Paper, Z -> Scissors
    // scored: Rock -> 1, Paper -> 2, Scissors -> 3
    // extra scored: lost -> 0, draw -> 3, win -> 6

    let file_path = "day2.txt";
    println!("Reading file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read this file");
    
    let plays = contents.split("\n");
    let mut sum = 0;
    for play in plays
    {
        if play == ""{
            continue;
        }
        let vec = play.split(" ").collect::<Vec<&str>>();
        println!("{:#?}", vec);
        let p1 = to_hands(&vec[0]);
        // let p2 = to_hands(&vec[1]);
        let p2 = define_hand(p1, &vec[1]);
        sum += get_winner(p1, p2);
        sum += get_hand(p2);
    }
    println!("sum = {sum}");
}

fn get_hand(player: Hands) -> i32
{
    match player {
        Hands::Rock => 1,
        Hands::Paper => 2,
        Hands::Scissors => 3,
    }
}

fn define_hand(player1: Hands, outcome: &str) -> Hands
{
    match outcome
    {
        "X" => { // lose
            match player1
            {
                Hands::Rock => Hands::Scissors,
                Hands::Paper => Hands::Rock,
                Hands::Scissors => Hands::Paper,
            }
        },
        
        "Z" => { // win
            match player1
            {
                Hands::Rock => Hands::Paper,
                Hands::Paper => Hands::Scissors,
                Hands::Scissors => Hands::Rock,
            }
        },
        _ => { // draw
            return player1.clone()
        },
    }
}

fn get_winner(p1: Hands, p2: Hands) -> i32
{
    match p1
    {
        Hands::Rock => {
            match p2 {
                Hands::Rock => return 3,
                Hands::Paper => return 6,
                Hands::Scissors => return 0,
            }
        },
        Hands::Paper => {
            match p2 {
                Hands::Rock => return 0,
                Hands::Paper => return 3,
                Hands::Scissors => return 6,
            }
        },
        Hands::Scissors => {
            match p2 {
                Hands::Rock => return 6,
                Hands::Paper => return 0,
                Hands::Scissors => return 3,
            }
        },
    }
}

fn to_hands(play: &str) -> Hands
{
    match play {
        "A" | "X" => return Hands::Rock,
        "B" | "Y"=> return Hands::Paper,
        "C" | "Z" => return Hands::Scissors,
        _ => {
            println!("this should not be possible: _{}_", play);
            exit(1);
        }
    };
}