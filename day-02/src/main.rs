use std::fs;
use std::env;

#[derive(Debug, PartialEq, Eq)]
enum SortOrder {
    Ascending,
    Descending,
}

fn get_input(file_path: &str, file_contents_vector: &mut Vec<Vec<usize>>) {
    
    let raw_file_contents = fs::read_to_string(file_path).unwrap();
    let mut file_contents_iter = raw_file_contents.lines();
    let mut line_str = file_contents_iter.next();
    
    while line_str != None {
        let mut line: Vec<usize> = vec![]; 
        let mut line_split_iter = line_str.unwrap().split(" ");
        let mut temp_split = line_split_iter.next();
        while temp_split != None {
            line.push(temp_split.unwrap().parse::<usize>().unwrap());
            temp_split = line_split_iter.next();
        }
        file_contents_vector.push(line);
        line_str = file_contents_iter.next();
    }

}

fn determine_order(vec: &Vec<usize>) -> Option<SortOrder> {

    let mut is_ascending = true;
    let mut is_descending = true;

    for i in 1..vec.len() {
        if vec[i] < vec[i - 1] {
            is_ascending = false;
            if vec[i-1] - vec[i] > 3 {
                return None;
            } 
        } else if vec[i] > vec[i - 1] {
            is_descending = false;
            if vec[i] - vec[i-1] > 3 {
                return None;
            }
        }
        else if vec[i] == vec[i-1]{
            return None;
        }

        if !is_ascending && !is_descending {
            return None;
        }
    }

    if is_ascending {
        //println!("{:?} asc sorted", vec);
        return Some(SortOrder::Ascending);
    } else if is_descending {
        //println!("{:?} desc sorted", vec);
        return Some(SortOrder::Descending);
    } else {
        return None;
    }
}

fn process_reports(file_contents_vector: &mut Vec<Vec<usize>>, no_of_safe_reports: &mut usize) {

    for index in 0..file_contents_vector.len() {
        if determine_order(&file_contents_vector[index]) != None {
            *no_of_safe_reports += 1;
        } else {
            for index_2 in 0..file_contents_vector[index].len() {
                let removed_value = file_contents_vector[index].remove(index_2);
                if determine_order(&file_contents_vector[index]) != None {
                    *no_of_safe_reports += 1;
                    break;
                } else {
                    file_contents_vector[index].insert(index_2, removed_value);
                }
            }
        }
    }
}

fn main() {

    env::set_var("RUST_BACKTRACE", "1");

    let mut no_of_safe_reports: usize = 0;
    let mut file_contents_vector: Vec<Vec<usize>> = vec![];

    //get_input("sample_input.txt", &mut file_contents_vector);
    get_input("test_input.txt", &mut file_contents_vector);

    process_reports(&mut file_contents_vector, &mut no_of_safe_reports);

    println!("{:?}", no_of_safe_reports);

}
