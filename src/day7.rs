use regex::Regex;
use std::{collections::HashMap, fs};

pub fn day7() {
    
    // detects lines with ls
    let ls  = Regex::new(r"\$\sls").unwrap();
    // detects lines with cd
    let cd  = Regex::new(r"\$\scd\s(\S*)").unwrap();
    // detects lines that contain a file (including size)
    let fil = Regex::new(r"(\d*)\s(\S*)").unwrap();
    // detects a directory including name
    let dir = Regex::new(r"dir\s(\S*)").unwrap();


    let file_contents = fs::read_to_string("day7.txt").unwrap();
    let lines = file_contents.split("\n");
    
    let mut current_path: Vec<String> = Vec::new();
    
    let mut dirs: HashMap<String, i32> = HashMap::from([(String::from("root"), 0)]);
    
    for line in lines
    {
        if ls.is_match(line)
        // ls match is found
        {
            
        }
        else if cd.is_match(line)
        // cd match is found
        {
            if line.contains("..")
            {
                current_path.pop();
                continue;
            }
            for mat in cd.captures_iter(line)
            {
                //dbg!(&mat);
                current_path.push(mat[1].to_string());
            }
        }
        else if dir.is_match(line)
        // directory is found
        {
            
        }
        else if fil.is_match(line)
        // file is found
        {
            for mat in fil.captures_iter(line)
            {
                //dbg!(&mat);
                for i in (0..current_path.len()+1).rev()
                {
                    if dirs.contains_key(&current_path[0..i].join("/"))
                    {
                        let value = dirs.get_mut(&current_path[0..i].join("/")).unwrap();
                        *value += mat[1].parse::<i32>().unwrap();
                    }
                    else
                    {
                        dirs.insert(current_path[0..i].join("/"), mat[1].parse::<i32>().unwrap());
                    }
                    //let value = dirs.entry(current_path[0..i].join("/"))
                    //    .or_insert(mat[1].parse::<i32>().unwrap());
                    //*value += mat[1].parse::<i32>().unwrap();
                }
            }
        }
        
        //dbg!(&current_path, &dirs);
    }
    let mut sum = 0;
    let min_free_size = 30_000_000;
    let tot_size = 70_000_000;
    let mut possible: Vec<i32> = Vec::new();
    
    
    let tot_in_use = dirs.get("root").unwrap();
    
    let min_size = min_free_size - (tot_size - tot_in_use);
    
    for (key, value) in dirs
    {
        // println!("{key} => {value}");
        if value < 100_000
        {
            sum += value;
        }
        if value > min_size
        {
            possible.push(value);
        }
    }
    
    // dbg!(&possible);
    
    let min = *possible.iter().min().unwrap();
    println!("min: {min}");
    println!("solution: {sum}");
}