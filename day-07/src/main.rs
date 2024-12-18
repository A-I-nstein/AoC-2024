use std::fs;
use std::env;

fn get_input(file_path: &str, calibration_equations: &mut Vec<(usize, Vec<usize>)>) {
    
    let raw_file_contents: String = fs::read_to_string(file_path).unwrap();
    let mut file_contents_iter = raw_file_contents.lines();
    let mut line_str = file_contents_iter.next();
    
    while line_str != None {
        let line: String = line_str.unwrap().to_string();
        let mut line_split = line.split(": ");
        let result: usize = line_split.next().unwrap().parse::<usize>().unwrap();
        let arguments: &str = line_split.next().unwrap();
        let mut arg_split = arguments.split(" ");
        let mut arg_vec: Vec<usize> = vec![];
        let mut temp_arg = arg_split.next();
        while temp_arg != None {
            arg_vec.push(temp_arg.unwrap().parse::<usize>().unwrap());
            temp_arg = arg_split.next();
        }
        calibration_equations.push((result, arg_vec));
        line_str = file_contents_iter.next();
    }
}

fn valid_result(equation: &(usize, Vec<usize>), operands_vec: &Vec<char>) -> bool {

    let result: usize = equation.0;
    let mut arguments  = equation.1.iter();
    let mut calculated_result: usize = *arguments.next().unwrap();


    for operand in operands_vec {
        //println!("{:?}", calculated_result);
        if *operand == '+' {
            calculated_result += *arguments.next().unwrap();
        } else if *operand == '*' {
            calculated_result *= *arguments.next().unwrap();
        } else {
            calculated_result = (calculated_result.to_string() + &(*arguments.next().unwrap()).to_string()).parse().unwrap();
        }
        if calculated_result > result {
            return false;
        }
    }

    if calculated_result == result {
        return true;
    } else {
        return false;
    }
}

fn generate_sample_space(n: &usize) -> Vec<Vec<char>> {
    let mut sample_space: Vec<Vec<char>> = vec![vec![]];

    for _ in 0..*n {
        let mut new_sample_space: Vec<Vec<char>> = Vec::new();
        for seq in sample_space {
            let mut new_seq: Vec<char> = seq.clone();
            new_seq.push('+');
            new_sample_space.push(new_seq);

            new_seq = seq.clone();
            new_seq.push('*');
            new_sample_space.push(new_seq);

            new_seq = seq.clone();
            new_seq.push('|');
            new_sample_space.push(new_seq);
        }
        sample_space = new_sample_space;
    }

    return sample_space;
}

fn bridge_repair(calibration_equations: &Vec<(usize, Vec<usize>)>) {
    let mut sum_of_valid_result: usize = 0;
    let mut count = 0;
    for equation in calibration_equations {

        let operand_sequences: Vec<Vec<char>> = generate_sample_space(&(equation.1.len()-1));
        for operand_vec in operand_sequences {
            if valid_result(equation, &operand_vec) {
                sum_of_valid_result += equation.0;
                break;
            }
        }
        println!("completed {}", count);
        count+=1;
    }
    println!("{}", sum_of_valid_result);
}

fn main() {

    env::set_var("RUST_BACKTRACE", "1");

    let mut calibration_equations: Vec<(usize, Vec<usize>)> = vec![];

    //get_input("sample_input.txt", &mut calibration_equations);
    get_input("test_input.txt", &mut calibration_equations);

    bridge_repair (&calibration_equations);

    // for item in calibration_equations {
    //     println!("{:?}", item);
    // }
    
}
