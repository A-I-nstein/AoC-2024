use std::fs;
use regex::Regex;

fn process_input_2(file_contents: &mut String, sum_of_products: &mut usize) {

    let re = Regex::new(r"mul\(\d{1,3},\s*\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let valid_blocks: Vec<&str> = re.find_iter(file_contents).map(|m| m.as_str()).collect();
    //println!("{:?}", valid_blocks);

    let valid_blocks_len = valid_blocks.len();
    
    let mut multiply = true;
    let mut index = 0;
    
    while index < valid_blocks_len {
        if valid_blocks[index].contains("do()") {
            multiply = true;
            index += 1;
        } else if valid_blocks[index].contains("don't()") {
            multiply = false;
            index += 1;
        } else if multiply {
            
            let re = Regex::new(r"\d{1,3},\s*\d{1,3}").unwrap();
            let multiplicand_multiplier: &str = re.find_iter(&valid_blocks[index]).map(|m| m.as_str()).collect::<Vec<_>>().pop().unwrap();
            //println!("{:?}", multiplicand_multiplier);
            let mut multiplicand_multiplier_split = multiplicand_multiplier.split(",");
            let multiplicand = multiplicand_multiplier_split.next().unwrap().parse::<usize>().unwrap();
            let multiplier = multiplicand_multiplier_split.next().unwrap().parse::<usize>().unwrap();
            
            *sum_of_products += multiplicand*multiplier;
            index += 1;
        } else {
            index += 1;
        }
    }
}

fn main() {

    let mut sum_of_products: usize = 0;
    
    //let file_path: &str = "sample_input.txt";
    let file_path: &str = "test_input.txt";

    let mut file_contents: String = fs::read_to_string(file_path).unwrap();

    process_input_2(&mut file_contents, &mut sum_of_products);

    println!("{:?}", sum_of_products);
}
