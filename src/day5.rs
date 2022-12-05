use std::fs;

pub fn day5()
{
    let file_name = "day5.txt";
    let contents = fs::read_to_string(file_name).expect("could not read input file");

    let lines = contents.split("\n");
    let mut data = false;

    let mut stacks: Vec<Stack> = Vec::new();

    stacks.push(Stack{
        name: 1,
        crates: vec![String::from("H"), String::from("C"), String::from("R")],
    });
    stacks.push(Stack{
        name: 2,
        crates: vec![String::from("B"), String::from("J"), String::from("H"), String::from("L"), String::from("S"), String::from("F")],
    });
    stacks.push(Stack{
        name: 3,
        crates: vec![String::from("R"), String::from("M"), String::from("D"), String::from("H"), String::from("J"), String::from("T"), String::from("Q")],
    });
    stacks.push(Stack{
        name: 4,
        crates: vec![String::from("S"), String::from("G"), String::from("R"), String::from("H"), String::from("Z"), String::from("B"), String::from("J")],
    });
    stacks.push(Stack{
        name: 5,
        crates: vec![String::from("R"), String::from("P"), String::from("F"), String::from("Z"), String::from("T"), String::from("D"), String::from("C"), String::from("B")],
    });
    stacks.push(Stack{
        name: 6,
        crates: vec![String::from("T"), String::from("H"), String::from("C"), String::from("G")],
    });
    stacks.push(Stack{
        name: 7,
        crates: vec![String::from("S"), String::from("N"), String::from("V"), String::from("Z"), String::from("B"), String::from("P"), String::from("W"), String::from("L")],
    });
    stacks.push(Stack{
        name: 8,
        crates: vec![String::from("R"), String::from("J"), String::from("Q"), String::from("G"), String::from("C")],
    });
    stacks.push(Stack{
        name: 9,
        crates: vec![String::from("L"), String::from("D"), String::from("T"), String::from("R"), String::from("H"), String::from("P"), String::from("F"), String::from("S")],
    });

    // s1.move_crates(&mut s2, 2);

    for line in lines
    {
        if line == ""
        {
            data = true;
            continue;
        }

        if data
        {
            // move 11 from 5 to 6
            // println!("{line}");
            let mut parts = line.split(" ");
            let amount = parts.nth(1).expect("could not get 1th element").parse::<i32>().unwrap();
            let origin = parts.nth(1).expect("could not get 3th element").parse::<i32>().unwrap();
            let destination = parts.nth(1).expect("could not get 5th element").parse::<i32>().unwrap();

            // println!("amount: {}, origin: {}, destination: {}",amount, origin, destination);
            // let or = stacks.get_mut(origin as usize - 1).expect("could not get origin out of stack");
            // let des = stacks.get_mut(destination as usize - 1).expect("could not get destination out of stack");

            let mut or: Option<&mut Stack> = None;
            let mut des: Option<&mut Stack> = None;
            let mut i = 1;
            for stack in &mut stacks
            {
                if origin == i
                {
                    or = Some(stack);
                }
                else if destination == i
                {
                    des = Some(stack);
                }
                i += 1;
            }
            // des.expect("des not right").move_crates(or.expect("or not right"), amount);
            des.expect("").move_multiple_crates(or.expect(""), amount);
        }
    }
    for stack in &stacks
    {
        stack.display();
    }
}

struct Stack
{
    name: i32,
    crates: Vec<String>
}

impl Stack
{
    // [x] initialise stack (including original stack)
    pub fn new(name: i32, crates: Vec<String>) -> Stack
    {
        Stack {crates, name}
    }

    // [x] display current stack + name
    pub fn display(&self)
    {
        print!("Stack {}", &self.name);
        println!("{:?}", &self.crates);
    }

    // [x] move crates (1 per 1) to another Stack
    pub fn move_crates(&mut self, other: &mut Stack, amount: i32)
    {
        for i in 0..amount
        {
            println!("self: {}, other: {}", self.name, other.name);
            // self.display();
            self.add_crate(other.pop_crate());
            // self.display();
        }
    }

    pub fn move_multiple_crates(&mut self, other: &mut Stack, amount: i32)
    {
        self.add_crates(&mut other.rem_crates(amount));
    }

    // [x] add crate (1 per 1)
    pub fn add_crate(&mut self, crates: Option<String>)
    {
        match crates
        {
            Some(c) => {
                println!("{}: added {}", self.name, &c);
                self.crates.push(c);
            },
            None => println!("could not add none crate to crates"),
        }
    }

    // [x] add multiple crates at once
    pub fn add_crates(&mut self, crates: &mut Vec<String>)
    {
        self.crates.append(crates);
    }

    // [x] pop crate (1 per 1)
    pub fn pop_crate(&mut self) -> Option<String>
    {
        self.crates.pop()
    }

    pub fn rem_crates(&mut self, amount: i32) -> Vec<String>
    {
        let mut ret: Vec<String> = Vec::<String>::new();
        println!("index: {}, self.len: {}, amount: {}", self.crates.len() - amount as usize, self.crates.len(), amount);
        for i in 0..amount
        {
            ret.push(self.crates.remove(self.crates.len() + i as usize - amount as usize));
        }
        ret
    }
}
