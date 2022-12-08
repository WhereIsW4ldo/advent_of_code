use std::fs;

use colored::Colorize;

fn check_biggest(forest: &Vec<Vec<u32>>, x: usize, y: usize, size: u32, direction: char) -> bool
{
    match direction
    {
        'l' => {

            for i in 0..x
            {
                if forest[y][i] >= size
                {
                    return false;
                }
            }
            return true;
        },
        'r' => {
            for i in x+1..forest[0].len()
            {
                if forest[y][i] >= size
                {
                    return false;
                }
            }
            return true;
        },
        'o' => {
            for j in y+1..forest.len()
            {
                if forest[j][x] >= size
                {
                    return false;
                }
            }
            return true;
        },
        'b' => {
            for j in 0..y
            {
                if forest[j][x] >= size
                {
                    return false;
                }
            }
            return true;
        },
        _ => return false,
    }
}

fn check_all(forest: &Vec<Vec<u32>>, x: usize, y: usize, size: u32) -> bool
{

    return check_biggest(&forest, x, y, size, 'l') 
        || check_biggest(&forest, x, y, size, 'r')
        || check_biggest(&forest, x, y, size, 'b')
        || check_biggest(&forest, x, y, size, 'o');

}

pub fn day8()
{
    let file_contents = fs::read_to_string(r"/home/waldo/Documents/AOC/day8.txt").unwrap();

    let lines = file_contents.split("\n").collect::<Vec<&str>>();


    let lines = vec!["30373", "25512", "65332", "33549", "35390"];
    
    let height = lines.len();
    let width  = lines.get(0).unwrap().len();

    println!("height: {height}, width: {width}");

    let mut forest = vec![vec![0; width]; height];

    println!("forest[0].len: {}, forest.len: {}", forest[0].len(), forest.len());

    for (i, line) in lines.iter().enumerate()
    {
        for (j, number) in line.chars().enumerate()
        {
            if number == '\n'
            {
                continue;
            }
            let nr = number.to_digit(10).unwrap();
            forest[i][j] = nr;
        }
    }    

    let mut sum = 0;

    for y in 0..forest.len()
    {
        for x in 0..forest[0].len()
        {
            let size = forest[y][x];
            
            if check_all(&forest, x, y, size)
            {
                print!("{}", format!("{}", size).blue());
                sum += 1;
            }
            else 
            {
                print!("{}", format!("{}", size).red());
            }
        }
        println!();
    }

    println!("sum: {sum}");
}