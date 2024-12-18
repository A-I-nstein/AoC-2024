use std::fs;

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

fn find_word_in_dir(grid: &Vec<Vec<char>>, n: isize, m: isize, word: &Vec<char>, index: usize, x: isize, y: isize, dir_x: isize, dir_y: isize) -> bool{
    if index == word.len() {
        return true;
    } else if is_valid(x, y, n, m) && word[index] == grid[x as usize][y as usize] {
        return find_word_in_dir(grid, n, m, word, index + 1, x + dir_x, y + dir_y, dir_x, dir_y);
    } else {
        return false
    }
}

#[derive(Debug)]
struct Mas {
    init_pos: (isize, isize),
    dir: (isize, isize),
}

fn ceres_search(grid: &Vec<Vec<char>>, mas_vec: &mut Vec<Mas>) {

    let word: Vec<char> = vec!['M', 'A', 'S'];
    
    let n: isize = grid.len() as isize;
    let m: isize = grid[0].len() as isize;
    let directions: [(isize, isize); 4] = [
        (1, 1), (1, -1), (-1, 1), (-1, -1)
    ];

    for i in 0..n {
        for j in 0..m {
            if grid[i as usize][j as usize] == word[0] {
                for (dir_x, dir_y) in directions {
                    if find_word_in_dir(grid, n, m, &word, 0, i, j, dir_x, dir_y) {
                        mas_vec.push(
                            Mas { init_pos: (i, j), dir: (dir_x, dir_y) }
                        )
                    }
                }
            }
        }
    }
}

fn find_x_mas(mas_vec: &Vec<Mas>, no_of_xmas: &mut usize) {

    let mut a_pos_vec: Vec<(isize, isize)> = vec![];
    for mas in mas_vec {
        let a_pos = (mas.init_pos.0 + mas.dir.0, mas.init_pos.1 + mas.dir.1);
        if a_pos_vec.contains(&a_pos) {
            *no_of_xmas += 1;
        } else {
            a_pos_vec.push(a_pos);
        }
    }
}

fn main() {

    let mut no_of_xmas: usize = 0;
    let mut mas_vec: Vec<Mas> = vec![];
    let mut file_contents_vector: Vec<Vec<char>> = vec![];

    get_input("sample_input.txt", &mut file_contents_vector);
    //get_input("test_input.txt", &mut file_contents_vector);

    ceres_search(&file_contents_vector, &mut mas_vec);
    find_x_mas(&mas_vec, &mut no_of_xmas);

    //println!("{:?}", file_contents_vector);
    //println!("{:?}", mas_vec)
    println!("{:?}", no_of_xmas);
}
