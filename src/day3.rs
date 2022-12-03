use std::fs;

pub fn day3()
{
    println!("day3 working");
    let file_name = "day3.txt";

    let contents = fs::read_to_string(file_name).expect("could not read file");

    // println!("{contents}");

    // lower and upper case are different items
    // each line is a list of items for each rucksack
    // both compartments are filled equally (for each rucksack)
    // sum of priorities a->z = 1->26, A->Z = 27->52
    // make sum of priorities for items that are contained in both halves
    
    let lines = contents.split("\n");

    let mut sum = 0;

    let mut first: &str = "";
    let mut second: &str = "";
    let mut third: &str = "";

    for (i, line) in lines.enumerate()
    {
        if line != ""
        {
            match i as i32%3
            {
                0 => first = line,
                1 => second = line,
                2 => third = line,
                _ => (),
            }
        }

        println!("first: {first}\n second: {second}\n third: {third}\n iter: {i}");
        if first != "" && second != "" && third != "" && i%3 == 2
        {
            let common_letters = search_letters(first, second, third);
            println!("commons: {:?}", common_letters);
            for letter in common_letters
            {
                sum += get_priority(letter);
            }
            println!("added to sum");
        }

        // Part 1
        // match line
        // {
        //     "" => (),
        //     _  => {
                
        //         // [ ] cut each line in 2 equally long strings
        //         let (first, second) = cut_string_2(line);

        //         // [ ] search common letters
        //         // let common_letters = search_letters(first, second);
                
        //         // [ ] calculate priority for common letters
                
        //         for letter in common_letters
        //         {
        //             sum += get_priority(letter);
        //             // println!("{letter} = {}", get_priority(letter));
        //         }
                
        //     }    
        // }
    }
    println!("{sum}");
}

fn get_priority(letter: char) -> u32
{
    let value = letter as u32;
    if value > 96
    {
        return value - 96;
    }
    else 
    {
        return value - 65 + 27;
    }
}

fn cut_string_2(original: &str) -> (&str, &str)
{
    original.split_at(original.len()/2)
}

// fn search_letters<'a>(first: &'a str, second: &'a str, third: &'a str, alread_found: Vec<char>) -> Vec<char>
fn search_letters<'a>(first: &'a str, second: &'a str, third: &'a str) -> Vec<char>
{
    let mut commons:Vec<char> = Vec::new();
    for letter1 in first.chars() 
    {
        for letter2 in second.chars()
        {
            for letter3 in third.chars()
            {
                match letter1 == letter2 && letter2 == letter3 && !commons.contains(&letter1){
                    true => {
                        commons.push(letter1);
                    },
                    false => (),
                }
            }
        }
    }
    // println!("{:?}", first_half);
    // println!("{:?}", second_half);
    println!("{:?}", commons);
    commons
}