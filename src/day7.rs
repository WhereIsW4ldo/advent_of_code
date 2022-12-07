use std::{fs, collections::HashMap, thread::current, hash::Hash};
use regex::Regex;

#[derive(Debug)]
struct Entry
{
    name: String,
    typ: Typ,
    size: i32,
    childs: Vec<Entry>,
}

impl Entry
{
    pub fn add_child(&mut self, parent: Vec<String>, new: Entry)
    {
        let mut parent_entry: &mut Entry = self;
        for dir in parent
        {

        }
        parent_entry.childs.push(new);
    }
}

#[derive(Debug)]
enum Typ
{
    File,
    Dir,
}

pub fn day7()
{
    let contents = fs::read_to_string("day7.txt").unwrap();


    let lines = contents.split("\n");

    // let com = Regex::new(r"\$").unwrap();
    let ls = Regex::new(r"\$\sls").unwrap();
    let cd = Regex::new(r"\$\scd\s(\S*)").unwrap();

    let dir = Regex::new(r"dir\s(\S*)").unwrap();

    let fil = Regex::new(r"(\d*)\s(\S*)").unwrap();

    let mut current_name: Vec<String> = Vec::new();

    let mut map: HashMap<String, i32> = HashMap::new();

    // let mut root: Entry = Entry { name: String::from("/"), typ: Typ::Dir, size: 0, childs: Vec::new() };

    let mut i = 0;

    for line in lines
    {
        // println!("{line}");
        if cd.is_match(line)
        // line is change directory
        {
            if line.contains("..")
            {
                current_name.pop();
            }
            else
            {
                for name in cd.captures_iter(line)
                {
                    // dbg!(&name);
                    current_name.push(name[1].to_string());
                }    
            }
        }

        else if !ls.is_match(line)
        // line is output of ls
        {
            if dir.is_match(line)
            {
                continue;
            }

            for file in fil.captures_iter(line)
            {
                current_name.push(file[2].to_string());
                map.insert(current_name.join("/"), file[1].parse::<i32>().unwrap());
                current_name.pop();
            }
        }  
    }
    // dbg!(current_name, map);

    let mut map_size: HashMap<String, i32> = HashMap::new();
    let mut map_size2: HashMap<String, i32> = HashMap::new();

    let mut dirs: Vec<String> = Vec::new();

    for (key, value) in map
    {
        for k in key.split("/").collect::<Vec<&str>>().split_last().unwrap().1.to_vec(){
            if !dirs.contains(&k.to_string())
            {
                dirs.push(k.to_string());
            }
        }
        // println!("{key} = {value}");
        let parent = key.split("/").collect::<Vec<&str>>().split_last().unwrap().1.join("/");
        match map_size.contains_key(&parent)
        {
            true => {
                let size = map_size.get_mut(&parent).unwrap();
                *size += value;
            },
            false => {
                map_size.insert(parent, value);
            },
        }
    }

    dbg!(&map_size, &dirs);

    let mut sum = 0;
    let mut tot_sum = 0;

    for d in dirs
    {
        sum = 0;
        for (dir, size) in &map_size
        {
            if dir.contains(&d)
            {
                if size <= &100000
                {
                    sum += size;
                }
            }
        }
        if sum < 100000
        {

        }
        println!("{d} = {sum}");
    }
}