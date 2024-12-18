use std::fs;

fn get_input(file_path: &str, file_contents_vector: &mut Vec<Vec<char>>, init_pos: &mut (isize, isize)) {
    
    let raw_file_contents: String = fs::read_to_string(file_path).unwrap();
    let mut file_contents_iter = raw_file_contents.lines();
    let mut line_str = file_contents_iter.next();
    let mut y: isize = 0;
    while line_str != None {
        let x:usize;
        let line: Vec<char> = line_str.unwrap().chars().collect();
        if line.contains(&'^') {
            x = line.iter().position(|&r| r == '^').unwrap();
            *init_pos = (x as isize, y);
        }
        file_contents_vector.push(line);
        line_str = file_contents_iter.next();
        y += 1;
    }
}

fn is_valid(x: isize, y: isize, size_x: isize, size_y: isize) -> bool {
    if  x >= 0 && x < size_x && y >= 0 && y < size_y {
        return true;
    } else {
        return false;
    }
}

fn form_quadrant(coordinates: &[(isize, isize)]) -> (isize, isize) {
    let point_4: (isize, isize) = (
        coordinates[0].0 + coordinates[2].0 - coordinates[1].0,
        coordinates[0].1 + coordinates[2].1 - coordinates[1].1
    );
    return point_4;
}

fn guard_gallivant(grid: &Vec<Vec<char>>, distinct_positions_vec: &mut Vec<(isize, isize)>, init_pos: &(isize, isize)) {
    
    let n: isize = grid.len() as isize; // y
    let m: isize = grid[0].len() as isize; // x
    let mut curr_pos: (isize, isize) = *init_pos;

    let mut obstacles: Vec<(isize, isize)> = vec![];

    let directions: [(isize, isize); 4] = [
        (0, -1), (1, 0), (0, 1), (-1, 0)
    ];
    let mut curr_dir: usize = 0;
    
    let mut next_pos: (isize, isize) = (curr_pos.0 + directions[curr_dir].0, curr_pos.1 + directions[curr_dir].1);
    if !distinct_positions_vec.contains(&curr_pos) {
        distinct_positions_vec.push(curr_pos);
    }
    // println!("x = {}, y = {}", m, n);
    // return;
    while is_valid(next_pos.0, next_pos.1, m, n) {
        //println!("{:?}", next_pos);
        if grid[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            obstacles.insert(0, (next_pos.0, next_pos.1));
            curr_dir = (curr_dir+1)%4 ;
            if obstacles.len() >= 3 {
                let point_4: (isize, isize) = form_quadrant(obstacles.split_at(3).0);
                if is_valid(point_4.0, point_4.1, m, n){
                    let mut temp_x: isize = curr_pos.0;
                    let mut temp_y: isize = curr_pos.1;
                    let mut obstacle_found = false;
                    while temp_x != point_4.0 || temp_y != point_4.1 {
                        temp_x = temp_x + directions[curr_dir].0; 
                        temp_y = temp_y + directions[curr_dir].1;
                        if grid[temp_y as usize][temp_x as usize] == '#' {
                            obstacle_found = true;
                            println!("{:?} found", point_4);
                            break;
                        }
                    }
                    if !obstacle_found {
                        println!("{:?}", point_4);
                    }
                }
            }
        } else {
            curr_pos = next_pos;
            if !distinct_positions_vec.contains(&curr_pos) {
                distinct_positions_vec.push(curr_pos);
            }
        }
        next_pos = (curr_pos.0 + directions[curr_dir].0, curr_pos.1 + directions[curr_dir].1);
    }
    //println!("{:?}", next_pos);
}

fn main() {

    let mut init_pos: (isize, isize) = (0, 0);
    let mut distinct_positions_vec: Vec<(isize, isize)> = vec![];
    let mut file_contents_vector: Vec<Vec<char>> = vec![];

    get_input("sample_input.txt", &mut file_contents_vector, &mut init_pos);
    //get_input("test_input.txt", &mut file_contents_vector, &mut init_pos);
    //get_input("deepak_test_input.txt", &mut file_contents_vector, &mut init_pos);

    guard_gallivant(&file_contents_vector, &mut distinct_positions_vec, &init_pos);
    //find_x_mas(&mas_vec, &mut no_of_xmas);

    //println!("{:?}", file_contents_vector);
    println!("{:?}", distinct_positions_vec.len());

}
