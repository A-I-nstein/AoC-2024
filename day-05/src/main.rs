use std::fs;

fn get_input(file_path: &str, page_order: &mut Vec<(usize, usize)>, updates: &mut Vec<Vec<usize>>) {
    
    let raw_file_contents: String = fs::read_to_string(file_path).unwrap();
    let mut file_contents_iter = raw_file_contents.lines();
    let mut line_str = file_contents_iter.next();

    let mut split_flag = false;
    
    while line_str != None {
        let line: String = line_str.unwrap().to_string();
        if "\n".contains(&line) {
            split_flag = true;
            line_str = file_contents_iter.next();
            continue;
        }
        if !split_flag {
            let mut line_split = line.split("|");
            let ele: (usize, usize) = (line_split.next().unwrap().parse::<usize>().unwrap(), line_split.next().unwrap().parse::<usize>().unwrap());
            page_order.push(ele);
        } else {
            let mut line_split = line.split(",");
            let mut line_ele: Vec<usize> = vec![];
            let mut ele: Option<&str> = line_split.next();
            while ele != None {
                line_ele.push(ele.unwrap().parse::<usize>().unwrap());
                ele = line_split.next();
            }
            updates.push(line_ele);
        }
        line_str = file_contents_iter.next();
    }
}

fn print_queue(page_order: &Vec<(usize, usize)>, updates: &mut Vec<Vec<usize>>, sum_of_valid_mid: &mut usize) {
    for update in updates {
        let update_len:usize = update.len();
        let mut flag = true;
        for i in 0..update_len {
            for j in i+1..update_len {
                //println!("{:?}", (update[i].clone(), update[j].clone()));
                if !page_order.contains(&(update[i].clone(), update[j].clone())) {
                    let temp = update[i];
                    update[i] = update[j];
                    update[j] = temp;
                    flag = false;
                }
            }
        }
        if flag {
            //println!("{:?}", update);
        }
        if !flag {
            println!("{:?}", update);
            *sum_of_valid_mid += update[update_len/2];
        }
    }
}

fn main() {

    let mut page_order: Vec<(usize, usize)> = vec![];
    let mut updates: Vec<Vec<usize>> = vec![];
    let mut sum_of_valid_mid: usize = 0;

    //get_input("sample_input.txt", &mut page_order, &mut updates);
    get_input("test_input.txt", &mut page_order, &mut updates);

    print_queue(&page_order, &mut updates, &mut sum_of_valid_mid);

    //println!("{:?}", page_order);
    //println!("{:?}", updates);
    println!("{:?}", sum_of_valid_mid);
}
