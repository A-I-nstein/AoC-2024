use std::fs;
use tree_ds::prelude::*;

fn get_input(file_path: &str, topographic_map: &mut Vec<Vec<usize>>) {
    
    let raw_file_contents: String = fs::read_to_string(file_path).unwrap();
    let mut file_contents_iter = raw_file_contents.lines();
    let mut line_str = file_contents_iter.next();
    
    while line_str != None {
        let line: Vec<char> = line_str.unwrap().chars().collect();
        let mut line_int: Vec<usize> = vec![];
        for i in line {
            line_int.push(i.to_string().parse().unwrap());
        }
        topographic_map.push(line_int);
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

fn find_all_valid_next_points (topographic_map: &Vec<Vec<usize>>, root_element: (usize, usize, usize), size_x: usize, size_y: usize) -> Vec<(usize, usize, usize)>{

    //println!("{:?}",root_element);

    let directions: [(isize, isize); 4] = [
        (0, -1), (1, 0), (0, 1), (-1, 0)
    ];
    
    let mut next_points: Vec<(usize, usize, usize)> = vec![];

    for direction in directions {
        let new_ele: (isize, isize) = (root_element.0 as isize + direction.0 as isize , root_element.1 as isize + direction.1 as isize);
        //println!("{:?}",new_ele);
        if is_valid(new_ele.1, new_ele.0, size_x as isize, size_y as isize) {       
            let new_value = topographic_map[new_ele.0 as usize][new_ele.1 as usize];
            if new_value == root_element.2 + 1 {
                next_points.push((new_ele.0 as usize, new_ele.1 as usize, new_value));
            }
        }
    }

    //println!("{:?}", next_points);
    return next_points;

}

fn hoof_it (topographic_map: &Vec<Vec<usize>>) {

    let mut trailhead_no: usize = 0;
    let mut trailhead_locs: Vec<(usize, usize, usize)> = vec![];
    let mut trailhead_scores: Vec<usize> = vec![];
    let mut trailhead_scores_sum: usize = 0;

    let size_y: usize = topographic_map.len();
    let size_x: usize = topographic_map[0].len();

    let mut processing_stack: Vec<(usize, usize, usize)> = vec![];

    for i_y in 0..size_y {
        for j_x in 0..size_x {
            if topographic_map[i_y][j_x] == 0 {
                trailhead_no += 1;
                trailhead_locs.push((i_y, j_x, 0))
            }
        } 
    }

    //println!("{} {:?}", trailhead_no, trailhead_locs);

    for i in 0..trailhead_locs.len() {

        let mut trailhead_score: usize = 0;
        let mut trainend_locs: Vec<(usize, usize, usize)> = vec![];

        processing_stack.push(trailhead_locs[i]);
        while processing_stack.len() != 0 {
            // println!("{:?}", processing_stack);
            let next_ele = processing_stack.pop().unwrap();
            if next_ele.2 == 9 {
                // if !trainend_locs.contains(&next_ele) {
                //     trainend_locs.push(next_ele);
                //     trailhead_score += 1;
                // }
                trailhead_score += 1;
                
            }
            let next_vec: Vec<(usize, usize, usize)> = find_all_valid_next_points(&topographic_map, next_ele, size_x, size_y);
            for next_ele in next_vec {
                processing_stack.push(next_ele);
            }
        }
        trailhead_scores.push(trailhead_score);
    }

    

    for i in trailhead_scores {
        trailhead_scores_sum += i;
    }

    println!("{:?}", trailhead_scores_sum);


}

fn main() {

    let mut topographic_map: Vec<Vec<usize>> = vec![];

    // get_input("sample_input.txt", &mut topographic_map);
    get_input("test_input.txt", &mut topographic_map);

    hoof_it(&topographic_map);

    // println!("{:?}", topographic_map);
}
