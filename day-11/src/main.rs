use std::fs;
use std::env;
use std::collections::HashMap;

fn get_input (file_path: &str, stones: &mut Vec<String>) {
    
    let raw_file_contents: String = fs::read_to_string(file_path).unwrap();
    let mut file_contents_iter: std::str::Lines<'_> = raw_file_contents.lines();
    let line_str: Option<&str> = file_contents_iter.next();
    let mut line_split_iter: std::str::Split<'_, &str> = line_str.unwrap().split(" ");
    let mut temp_split: Option<&str> = line_split_iter.next();
    while temp_split != None {
        stones.push(temp_split.unwrap().to_string());
        temp_split = line_split_iter.next();
    }
}

fn blink_for (no_of_times: usize, stones: &Vec<String>) {
    
    // println!("{:?}", stones);

    let mut stone_arrangement: Vec<String> = stones.clone();

    for blink in 0..no_of_times {
        println!("blinked {} times", blink+1);
        let mut new_arrangement: Vec<String> = vec![];
        let mut stone_iter = stone_arrangement.iter();
        let mut stone = stone_iter.next();
        while stone != None {
            if *stone.unwrap() == "0" {
                new_arrangement.push("1".to_string());
            } else {
                if stone.unwrap().len() % 2 == 0 {
                    let (first, last) = stone.unwrap().split_at(stone.unwrap().len()/2);
                    new_arrangement.push(first.to_string());
                    new_arrangement.push(last.to_string().parse::<usize>().unwrap().to_string());
                } else {
                    new_arrangement.push((stone.unwrap().to_string().parse::<usize>().unwrap() * 2024).to_string());
                }
            }
            stone = stone_iter.next();
        }
        stone_arrangement = new_arrangement.clone();
        println!("{} {:?}", stone_arrangement.len(), stone_arrangement);
    }

    // println!("{:?}", stone_arrangement.len());

}

fn blink_2 (no_of_times: usize, stones: &Vec<String>) {

    let mut stone_arrangement: HashMap<String, usize> = HashMap::new();

    for i in stones {
        stone_arrangement.insert(i.to_string(), 1);
    }

    for blink in 0..no_of_times {
        //println!("blinked {} times", blink+1);
        let mut new_arrangement: HashMap<String, usize> = HashMap::new();

        for (value, old_count) in stone_arrangement {

            if value == "0" {
                let count = new_arrangement.entry("1".to_string()).or_default();
                *count += old_count;
            } else {
                if value.len() % 2 == 0 {
                    let (first, last) = value.split_at(value.len()/2);

                    let first_mod = first.to_string().parse::<usize>().unwrap().to_string();
                    // println!("first {}", first_mod);
                    let count = new_arrangement.entry(first_mod).or_default();
                    // println!("{}", count);
                    *count += old_count;

                    let last_mod = last.to_string().parse::<usize>().unwrap().to_string();
                    // println!("last {}", last_mod);
                    let count = new_arrangement.entry(last_mod).or_default();
                    // println!("{}", count);
                    *count += old_count;


                } else {
                    let count = new_arrangement.entry((value.to_string().parse::<usize>().unwrap() * 2024).to_string()).or_default();
                    *count += old_count;
                }
            }

        }

        stone_arrangement = new_arrangement.clone();
        // println!("{} {:?}", stone_arrangement.len(), stone_arrangement);

    }

    let mut sum : usize = 0;
    for i in stone_arrangement.values().into_iter() {
        sum += *i;
    }
    println!("{}", sum);


}


fn main() {

    env::set_var("RUST_BACKTRACE", "1");

    let mut stones: Vec<String> = vec![];

    // get_input("sample_input.txt", &mut stones);
    get_input("test_input.txt", &mut stones);

    blink_2(75, &stones);

}