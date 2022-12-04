use std::fs;

pub fn day4()
{
    let file_name = "day4.txt";
    let file_contents = fs::read_to_string(file_name).expect("could not read input file");

    // [x] loop through file_contents
    let lines = file_contents.split("\n");
    let mut sum = 0;
    for line in lines
    {
        if line == ""
        {
            continue;
        }
        // dbg!(line);
        // [x] split line in multiple couples of 5-6 on a ','
        let section_assignment: Vec<&str> = line.split(",").collect();
        
        // [x] loop through each section assignment and get first and second value
        let values1: Vec<&str> = section_assignment[0].split("-").collect();
        let values2: Vec<&str> = section_assignment[1].split("-").collect();
        // part 1:
        // if fully_contains(&values1, &values2)
        // {
        //     // dbg!(&values1);
        //     // dbg!(&values2);
        //     sum += 1;
        // }
        
        // part 2:
        if overlap(&values1, &values2)
        {
            println!(" values1: {:?}", &values1);
            println!(" values2: {:?}", &values2);
            sum += 1;
        }
    }
    println!("sum = {sum}");
}

fn fully_contains(values1: &Vec<&str>, values2: &Vec<&str>) -> bool
{
    return ((values2[0].parse::<i32>().unwrap() <= values1[0].parse::<i32>().unwrap()) && (values1[1].parse::<i32>().unwrap() <= values2[1].parse::<i32>().unwrap())) || ((values1[0].parse::<i32>().unwrap() <= values2[0].parse::<i32>().unwrap()) && (values2[1].parse::<i32>().unwrap() <= values1[1].parse::<i32>().unwrap()))
}

fn overlap(values1: &Vec<&str>, values2: &Vec<&str>) -> bool
{
    let s1 = values1[0].parse::<i32>().unwrap();
    let e1 = values1[1].parse::<i32>().unwrap();
    let s2 = values2[0].parse::<i32>().unwrap();
    let e2 = values2[1].parse::<i32>().unwrap();
    // if fully_contains(values1, values2)
    // {
    //     return true;
    // }
    for i in s1..e1+1
    {
        for j in s2..e2+1
        {
            if i == j
            {
                return true;
            }
        }
    }
    return false;
}