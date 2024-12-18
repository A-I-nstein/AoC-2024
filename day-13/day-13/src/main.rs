use std::fs;
use std::collections::HashSet;
use ndarray::prelude::*;
use ndarray_linalg::Solve;

#[derive(Debug)]
struct ClawMachine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize_location: (usize, usize)
}

fn get_button_delta (line: &str) -> (usize, usize) {

    let dx: usize;
    let dy: usize;

    let line_split: Vec<&str> = line.split(":").collect();
    dx = line_split[1].split(",").collect::<Vec<_>>()[0].split("+").collect::<Vec<_>>()[1].parse::<usize>().unwrap();
    dy = line_split[1].split(",").collect::<Vec<_>>()[1].split("+").collect::<Vec<_>>()[1].parse::<usize>().unwrap();

    return (dx, dy);
}

fn get_prize_location (line: &str) -> (usize, usize) {

    let x: usize;
    let y: usize;

    let line_split: Vec<&str> = line.split(":").collect();
    x = 10000000000000 + line_split[1].split(",").collect::<Vec<_>>()[0].split("=").collect::<Vec<_>>()[1].parse::<usize>().unwrap();
    y = 10000000000000 + line_split[1].split(",").collect::<Vec<_>>()[1].split("=").collect::<Vec<_>>()[1].parse::<usize>().unwrap();

    return (x, y);
}

fn get_input (file_path: &str) -> Vec<ClawMachine> {

    let mut claw_machines_vec: Vec<ClawMachine> = vec![];

    let file_contents: String = fs::read_to_string(file_path).unwrap();
    let file_contents_vec: Vec<&str> = file_contents.lines().collect();
    let no_of_lines: usize = file_contents_vec.len();

    for line_index in (0..no_of_lines).step_by(4) {
        let claw_machine = ClawMachine {
            button_a: get_button_delta(file_contents_vec[line_index]),
            button_b: get_button_delta(file_contents_vec[line_index+1]),
            prize_location: get_prize_location(file_contents_vec[line_index+2])
        };
        claw_machines_vec.push(claw_machine);
    }

    return claw_machines_vec;

}

fn claw_contraption(claw_machine: ClawMachine) -> (usize, usize) {

    let mut procesing_stack: Vec<(usize, usize, usize, usize)> = vec![];
    let mut visited_set: HashSet<(usize, usize)> = HashSet::new();
    let button_b_count: usize;    

    procesing_stack.push((0, 0, 1, 0));
    let buttons: [(usize, usize); 2] = [claw_machine.button_a, claw_machine.button_b];
    
    while procesing_stack.len() != 0 {

        let (x, y, depth, a_count) = procesing_stack.remove(0);

        if (x, y) == claw_machine.prize_location {
            button_b_count = depth - a_count - 1;
            return (a_count, button_b_count);
        } else {
            for n in 0..buttons.len() {
                let (new_x, new_y) = (x + buttons[n].0, y + buttons[n].1);
                let new_a_count: usize;
                if n == 0 {
                    new_a_count = a_count + 1;
                } else {
                    new_a_count = a_count;
                }
                if !visited_set.contains(&(new_x, new_y)) {
                    visited_set.insert((new_x, new_y));
                    procesing_stack.push((new_x, new_y, depth+1, new_a_count));
                }

            }
        }
        
    }

    return (0, 0);

}

fn claw_contraption_2(claw_machine: ClawMachine) -> (usize, usize) {

    let a_count: usize = 0;
    let b_count: usize = 0;

    let a: Array2<f64> = array![[3., 2., -1.], [2., -2., 4.], [-2., 1., -2.]];
    let b: Array1<f64> = array![1., -2., 0.];
    let x = a.solve_into(b).unwrap();


    return  (a_count, b_count);

}

fn main () {

    let claw_machine_vec: Vec<ClawMachine> = get_input("input_2.txt");
    let mut result: usize = 0;
    // println!("{:?}", claw_machine_vec);

    for claw_machine in claw_machine_vec {
        // println!("{:?}", claw_contraption(claw_machine));
        // let (a, b) = claw_contraption(claw_machine);

        let (a, b) = claw_contraption_2(claw_machine);

        result += a * 3 + b * 1;
    }

    println!("{:?}", result);
}