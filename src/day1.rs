use std::fs;

pub fn day1(){
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let ar_input = contents.split("\n");
    let mut elves: Vec<(i32, i32)> = Vec::new();
    let mut sum = 0;
    for (i, input) in ar_input.enumerate() 
    {
        if input == ""
        {
            elves.push((i as i32, sum));
            sum = 0;
        }
        else
        {
            let temp: i32 = input.parse().unwrap();
            sum = sum + temp;
        }
    }
    let mut max = 0;
    let mut second = 0;
    let mut third = 0;
    for elf in elves{
        if elf.1 > max
        {
            third = second;
            second = max;
            max = elf.1;
        }
        else if elf.1 > second
        {
            third = second;
            second = elf.1;
        }
        else if elf.1 > third 
        {
            third = elf.1;    
        }
    }
    println!("Max = {max}");
    println!("Second = {second}");
    println!("Third = {third}");
    println!("sum = {}", max + second + third)
}