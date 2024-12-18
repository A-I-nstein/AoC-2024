use std::fs;

fn main () {

    let mut distance_vec: Vec<isize> = vec![];
    let input_file_path =  "src\\input.txt";
    let mut location_id_vec_1: Vec<usize> = vec![];
    let mut location_id_vec_2: Vec<usize> = vec![];

    let input_vector: Vec<_> = fs::read_to_string(input_file_path).unwrap().lines().map(String::from).collect();
    let input_vector_length = input_vector.len();

    for index in 0..input_vector_length {
        let mut parts = input_vector[index].split("   ");
        location_id_vec_1.push(parts.next().expect("REASON").parse::<usize>().expect("REASON"));
        location_id_vec_2.push(parts.next().expect("REASON").parse::<usize>().expect("REASON"));
    }

    location_id_vec_1.sort();
    location_id_vec_2.sort();

    for index in 0..input_vector_length {
        let distance = location_id_vec_1[index] - location_id_vec_2[index];
        if distance > 0 {
            distance_vec.push(distance as isize);
        }
        else {
            distance_vec.push(-(distance as isize));
        }
    }

    let sum: isize = distance_vec.iter().sum();
    println!("{:?}", sum);

    let input_vector_length = location_id_vec_1.len();

    let mut similarity_score_vec = vec![];
    for index in 0..input_vector_length {
        let similarity_score = location_id_vec_2.iter().filter(|&n| *n == location_id_vec_1[index]).count();
        similarity_score_vec.push(similarity_score*location_id_vec_1[index]);
    }

    let sum: usize = similarity_score_vec.iter().sum();
    println!("{:?}", sum);
}
