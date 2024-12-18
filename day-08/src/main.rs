use std::fs;
use std::env;
use::std::collections::HashMap;

fn get_input(file_path: &str, file_contents_vector: &mut Vec<Vec<char>>) {
    
    let raw_file_contents: String = fs::read_to_string(file_path).unwrap();
    let mut file_contents_iter = raw_file_contents.lines();
    let mut line_str = file_contents_iter.next();
    while line_str != None {
        let line: Vec<char> = line_str.unwrap().chars().collect();
        file_contents_vector.push(line);
        line_str = file_contents_iter.next();
    }
}

fn is_valid(x: isize, y: isize, size_x: isize, size_y: isize) -> bool {
    if  x >= 0 && x < size_x && y >= 0 && y < size_y {
        return true;
    } else {
        return false;
    }
}

fn find_antinode_locs (antenna_1_loc: (isize, isize), antenna_2_loc: (isize, isize), unique_antinode_loc_vec: &mut Vec<(isize, isize)>, size_x:isize, size_y:isize) {
    
    if !unique_antinode_loc_vec.contains(&antenna_1_loc) {
        //println!("{:?}", antenna_1_loc);
        unique_antinode_loc_vec.push(antenna_1_loc);
    }

    if !unique_antinode_loc_vec.contains(&antenna_2_loc) {
        //println!("{:?}", antenna_2_loc);
        unique_antinode_loc_vec.push(antenna_2_loc);
    }

    let delta_1: (isize, isize) = (antenna_1_loc.0 - antenna_2_loc.0, antenna_1_loc.1 - antenna_2_loc.1);
    let mut new_antinode_loc: (isize, isize) = (antenna_1_loc.0 + delta_1.0, antenna_1_loc.1 + delta_1.1);

    while is_valid(new_antinode_loc.1, new_antinode_loc.0, size_x, size_y) {
        if !unique_antinode_loc_vec.contains(&new_antinode_loc) {
            //println!("{:?}", new_antinode_loc);
            unique_antinode_loc_vec.push(new_antinode_loc);
        }
        new_antinode_loc = (new_antinode_loc.0 + delta_1.0, new_antinode_loc.1 + delta_1.1);
    }

    let delta_2: (isize, isize) = (antenna_2_loc.0 - antenna_1_loc.0, antenna_2_loc.1 - antenna_1_loc.1);
    let mut new_antinode_loc: (isize, isize) = (antenna_2_loc.0 + delta_2.0, antenna_2_loc.1 + delta_2.1);

    while is_valid(new_antinode_loc.1, new_antinode_loc.0, size_x, size_y) {
        if !unique_antinode_loc_vec.contains(&new_antinode_loc) {
            //println!("{:?}", new_antinode_loc);
            unique_antinode_loc_vec.push(new_antinode_loc);
        }
        new_antinode_loc = (new_antinode_loc.0 + delta_2.0, new_antinode_loc.1 + delta_2.1);
    }

}

fn resonant_collinearity(file_contents_vector: &Vec<Vec<char>>, unique_antinode_loc_vec: &mut Vec<(isize, isize)>) {
    
    let size_y: usize = file_contents_vector.len();
    let size_x: usize = file_contents_vector[0].len();
    let mut antenna_locations: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for i in 0..size_y {
        for j in 0..size_x {
            if file_contents_vector[i][j] != '.' {
                let specific_antenna_vec: &mut Vec<(isize, isize)> = antenna_locations.entry(file_contents_vector[i][j]).or_insert(vec![]);
                specific_antenna_vec.push((i as isize, j as isize));
            }
        }
    }
    //println!("{:?}", antenna_locations);

    for antenna in antenna_locations {
        let specific_antenna_locs: Vec<(isize, isize)> = antenna.1;
        let specific_antenna_locs_len: usize = specific_antenna_locs.len();
        for index_1 in 0..specific_antenna_locs_len {
            for index_2 in index_1+1..specific_antenna_locs_len {
                //println!("{:?} {:?}", specific_antenna_locs[index_1], specific_antenna_locs[index_2]);
                find_antinode_locs(specific_antenna_locs[index_1], specific_antenna_locs[index_2], unique_antinode_loc_vec, size_x as isize, size_y as isize);
            }
        }
    }

}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut unique_antinode_loc_vec: Vec<(isize, isize)> = vec![];
    let mut file_contents_vector: Vec<Vec<char>> = vec![];

    //get_input("sample_input.txt", &mut file_contents_vector);
    get_input("test_input.txt", &mut file_contents_vector);

    resonant_collinearity(&file_contents_vector, &mut unique_antinode_loc_vec);

    //println!("{:?}", file_contents_vector);
    //println!("{:?}", unique_antinode_loc_vec);
    println!("{:?}", unique_antinode_loc_vec.len());

    // for i in unique_antinode_loc_vec {
    //     file_contents_vector[i.0 as usize][i.1 as usize] = '#';
    // }

    //println!("{:?}", file_contents_vector);

}
