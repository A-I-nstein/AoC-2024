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

fn check_loop(grid: &Vec<Vec<char>>, init_pos: &(isize, isize)) -> bool {
    
    let n: isize = grid.len() as isize;
    let m: isize = grid[0].len() as isize;
    let mut curr_pos: (isize, isize) = *init_pos;
    let mut stuck_in_loop: bool = false;
    let mut distinct_positions_vec: Vec<((isize, isize), (isize, isize))> = vec![];

    let directions: [(isize, isize); 4] = [
        (0, -1), (1, 0), (0, 1), (-1, 0)
    ];
    let mut curr_dir: usize = 0;
    
    let mut next_pos: (isize, isize) = (curr_pos.0 + directions[curr_dir].0, curr_pos.1 + directions[curr_dir].1);
    
    while is_valid(next_pos.0, next_pos.1, m, n) {
        if grid[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            curr_dir = (curr_dir+1)%4 ;
        } else {
            curr_pos = next_pos;
            if !distinct_positions_vec.contains(&(curr_pos, directions[curr_dir])) {
                distinct_positions_vec.push((curr_pos, directions[curr_dir]));
            } else {
                //println!("Stuck in loop!");
                stuck_in_loop = true;
                break;
            }
        }
        next_pos = (curr_pos.0 + directions[curr_dir].0, curr_pos.1 + directions[curr_dir].1);
    }
    return stuck_in_loop;
}

fn guard_gallivant (grid: &mut Vec<Vec<char>>, init_pos: &(isize, isize)) {
    
    let y: isize = grid.len() as isize;
    let x: isize = grid[0].len() as isize;
    let mut possible_loopy_obstacles: usize = 0;

    for i in 0..y {
        for j in 0..x {
            if grid[i as usize][j as usize] != '#' && grid[i as usize][j as usize] != '^' {
                grid[i as usize][j as usize] = '#';
                //println!("{:?}", grid);
                if check_loop(grid, init_pos) {
                    possible_loopy_obstacles += 1;
                    //println!("x = {}, y = {}", j, i);
                }
                grid[i as usize][j as usize] = '.';
            }
        }
        println!("Completed row {}", i);
    }
    println!("{}", possible_loopy_obstacles);
    //println!("{:?}", grid);
}

fn main() {

    let mut init_pos: (isize, isize) = (0, 0);
    let mut file_contents_vector: Vec<Vec<char>> = vec![];

    //get_input("sample_input.txt", &mut file_contents_vector, &mut init_pos);
    get_input("test_input.txt", &mut file_contents_vector, &mut init_pos);
    //println!("{:?}", file_contents_vector);

    guard_gallivant(&mut file_contents_vector, &init_pos);

    //println!("{:?}", file_contents_vector);

}
