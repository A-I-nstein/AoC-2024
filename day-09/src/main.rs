use std::fs;
use std::env;

fn get_input(file_path: &str, disk_map: &mut Vec<char>) {
    
    let raw_file_contents: String = fs::read_to_string(file_path).unwrap();
    let mut file_contents_iter = raw_file_contents.lines();
    let line_str = file_contents_iter.next();
    let line: Vec<char> = line_str.unwrap().chars().collect();
    for char in line {
        disk_map.push(char);
    }
}

fn convert_disk_map_to_block_rep (disk_map: &Vec<char>, block_rep: &mut Vec<String>) {

    let mut data_block: bool = true;
    let mut data_block_id: usize = 0;
    let mut disk_map_iter = disk_map.iter();
    let mut block = disk_map_iter.next();

    while block != None {

        //println!("{}", String::from(*block.unwrap()).parse::<usize>().unwrap());
        let block_data: usize = String::from(*block.unwrap()).parse().unwrap();

        if data_block {
            for _i in 0..block_data {
                block_rep.push(data_block_id.to_string());
            }
            data_block_id += 1;
        } else {
            for _i in 0..block_data {
                block_rep.push('.'.to_string());
            }
        }

        data_block = !data_block;
        block = disk_map_iter.next();

    }

}

fn fragment_disk(block_rep: &mut Vec<String>) {
    
    let mut empty_index = 0;
    let mut value_index = block_rep.len()-1;
    
    while empty_index < value_index-1 {
        while block_rep[empty_index] != "." {
            empty_index += 1;
        }
        if block_rep[value_index] != "." {
            block_rep[empty_index] = block_rep[value_index].clone();
            block_rep[value_index] = ".".to_string();
        }
        value_index -= 1;
    }

}

fn fragment_disk_2(block_rep: &mut Vec<String>) {
    
    let mut curr_block_id: String = block_rep[block_rep.len()-1].clone();
    let mut value_index = block_rep.len()-1;
    
    while value_index > 0{
        let mut curr_block_len = 0;
        while block_rep[value_index] != curr_block_id {
            value_index -= 1;
        }
        // println!("curr_block_id {}", curr_block_id);
        // println!("value_index {}", value_index);
        while block_rep[value_index] == curr_block_id {
            value_index -= 1;
            curr_block_len += 1;
        }
        let mut empty_index = 0;
        while empty_index < value_index - curr_block_len {
            let mut space_block_len = 0;
            while block_rep[empty_index] != "." {
                empty_index += 1;
            }
            while block_rep[empty_index] == "." {
                space_block_len += 1;
                empty_index += 1;
            }
            if curr_block_len <= space_block_len {
                for i in 1..curr_block_len+1 {
                    //println!("swap {} {}", empty_index - space_block_len -1 + i, value_index+i);
                    block_rep.swap(empty_index - space_block_len -1 + i, value_index+i);
                }
                // println!("{:?} \n", block_rep);
                break;
            }
        }

        if curr_block_id != "1" {
            curr_block_id = (curr_block_id.parse::<usize>().unwrap() - 1).to_string();
        } else {
            break;
        }
    }

}

fn find_free_space (block_rep: &Vec<String>) -> Vec<(usize, usize)>{

    let mut free_space: Vec<(usize, usize)> = vec![];
    
    let mut index: usize = 0;
    let block_rep_len: usize = block_rep.len();

    while index < block_rep_len {
        if block_rep[index] != "." {
            while index < block_rep_len && block_rep[index] != "." {
                index += 1;
            }
        } else {
            let mut space_len = 0;
            while index < block_rep_len && block_rep[index] == "." {
                index += 1;
                space_len += 1;
            }
            free_space.push((index-space_len, space_len))
        }
    }

    return free_space;
}

fn find_file_locations (block_rep: &Vec<String>) -> Vec<(usize, usize)> {

    let mut file_locations: Vec<(usize, usize)> = vec![];
    
    let mut index: usize = 0;
    let block_rep_len: usize = block_rep.len();
    let mut block_id: String = "0".to_string();

    while index < block_rep_len {
        if block_rep[index] != "." {
            let mut block_len: usize = 0;
            while index < block_rep_len && block_rep[index] == block_id {
                block_len += 1;
                index += 1;
            }
            block_id = (block_id.parse::<usize>().unwrap() + 1).to_string();
            file_locations.push((index-block_len, block_len));
        } else {
            while index < block_rep_len && block_rep[index] == "." {
                index += 1;
            }
        }
    }
    return file_locations;

}

fn fragment_disk_3 (block_rep: &mut Vec<String>) {

    let mut free_space: Vec<(usize, usize)>;
    
    let file_locations: Vec<(usize, usize)> = find_file_locations(block_rep);

    let file_locs_len: usize = file_locations.len();
    let mut completed: usize = 0;

    for curr_file in file_locations.iter().rev() {
        free_space = find_free_space(block_rep);
        for space in free_space {
            if space.1 >= curr_file.1 && space.0 < curr_file.0 {
                for i in 0..curr_file.1 {
                    //println!("{} {}", space.0+i, curr_file.0+i);
                    block_rep.swap(space.0+i, curr_file.0+i);
                }
                //println!("{:?}", block_rep);
            }
        }
        completed += 1;
        println!("Completed {} of {}", completed, file_locs_len);
    }

    //println!("{:?}", block_rep);

}

fn calculate_check_sum(block_rep: &Vec<String>){
    
    let mut check_sum : usize = 0;
    let block_rep_len = block_rep.len();

    for i in 0..block_rep_len {
        if block_rep[i] == "." {
            continue;
        } else {
            check_sum += i * (block_rep[i].parse::<usize>().unwrap());
        }
    }
    println!("{}", check_sum);
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut disk_map: Vec<char> = vec![];
    let mut block_rep: Vec<String> = vec![];
    
    // get_input("sample_input.txt", &mut disk_map);
    get_input("test_input.txt", &mut disk_map);
    // println!("{:?}", disk_map);
    
    convert_disk_map_to_block_rep(&disk_map, &mut block_rep);
    //println!("{:?}", block_rep);
    
    fragment_disk_3(&mut block_rep);
    //println!("{:?}", block_rep[100]);

    calculate_check_sum(&block_rep);
    
    //println!("{:?}", block_rep);

}
