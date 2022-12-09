use std::fs;



pub fn day8()
{
    let file_contents = fs::read_to_string(r"/home/waldo/Documents/AOC/day8.txt").unwrap();

    let lines = file_contents.split("\n").collect::<Vec<&str>>();


    // let lines = vec!["30373", "25512", "65332", "33549", "35390"];
    
    let height = lines.len();
    let width  = lines.get(0).unwrap().len();

    println!("height: {height}, width: {width}");

    let mut forest = vec![vec![0; width]; height];

    println!("forest[0].len: {}, forest.len: {}", forest[0].len(), forest.len());

    for (i, line) in lines.iter().enumerate()
    {
        for (j, number) in line.chars().enumerate()
        {
            // if number == '\n'
            // {
            //     continue;
            // }
            let nr = number.to_digit(10).unwrap();
            forest[i][j] = nr;
            // print!("{}", nr);
        }
        // println!();
    }    

    let mut max_scenic_score = 0;

    for y in 0..forest.len()
    {
        for x in 0..forest[0].len()
        {
            let size = forest[y][x];
            
            // println!("{}", get_distance(&forest, x, y, size, 'o'));

            let score = distance_all(&forest, x, y, size);

            if max_scenic_score < score
            {
                max_scenic_score = score;
            }
            
            // if check_all(&forest, x, y, size)
            // {
            // //    print!("{}", size);
            //     sum += 1;
            // }
            // else 
            // {
            // //    print!("{}", size);
            // }
        }
        // println!();
    }

    println!("sum: {max_scenic_score}");
}

fn check_all(forest: &Vec<Vec<u32>>, x: usize, y: usize, size: u32) -> bool
{

    return check_biggest(&forest, x, y, size, 'l') 
        || check_biggest(&forest, x, y, size, 'r')
        || check_biggest(&forest, x, y, size, 'b')
        || check_biggest(&forest, x, y, size, 'o');

}

fn distance_all(forest: &Vec<Vec<u32>>, x: usize, y: usize, size: u32) -> i32
{

    return get_distance(&forest, x, y, size, 'l') 
        * get_distance(&forest, x, y, size, 'r')
        * get_distance(&forest, x, y, size, 'b')
        * get_distance(&forest, x, y, size, 'o');

}

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

fn get_distance(forest: &Vec<Vec<u32>>, x: usize, y: usize, size: u32, direction: char) -> i32
{
    let mut distance = 0;
    match direction
    {
        'l' => {
            for i in (0..x).rev()
            {
                distance += 1;
                if forest[y][i] >= size
                {
                    return distance;
                }
            }
        },
        'r' => {
            for i in x+1..forest[0].len()
            {
                distance += 1;
                if forest[y][i] >= size
                {
                    return distance;
                }
            }
        },
        'o' => {
            for j in y+1..forest.len()
            {
                distance += 1;
                if forest[j][x] >= size
                {
                    return distance;
                }
            }
        },
        'b' => {
            for j in (0..y).rev()
            {
                distance += 1;
                if forest[j][x] >= size
                {
                    return distance;
                }
            }
        },
        _ => (),
    }
    return distance;
}

