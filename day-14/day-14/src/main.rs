use std::fs;
use std::env;
use image::{ImageBuffer, Luma};

#[derive(Debug)]
struct Robot {
    curr_pos_x: isize,
    curr_pos_y: isize,
    velocity_x: isize,
    velocity_y: isize
}

impl Robot {

    fn move_next (&mut self, room: &mut Room) {

        self.curr_pos_x = (self.curr_pos_x + self.velocity_x + room.len_x as isize) % room.len_x as isize ;
        self.curr_pos_y = (self.curr_pos_y + self.velocity_y + room.len_y as isize) % room.len_y as isize ;

    }
    
}

#[derive(Debug)]
struct Room {
    len_x: usize,
    len_y: usize,
    time_elapsed: usize,
    robot_pos: Vec<Vec<usize>>
}

impl Room {

    fn new (len_x: usize, len_y: usize) -> Self {
        Self {
            len_x: len_x,
            len_y: len_y,
            time_elapsed: 0,
            robot_pos: vec![vec![0; len_x]; len_y]
        }
    }

    fn place_robots (&mut self, robots: &Vec<Robot>) {

        self.robot_pos = vec![vec![0; self.len_x]; self.len_y];

        for robot in robots {
            self.robot_pos[robot.curr_pos_y as usize][robot.curr_pos_x as usize] += 1;
        }

        // let mut data_file = OpenOptions::new()
        // .append(true)
        // .open("data".to_owned()+&iteration.to_string()+".png")
        // .expect("cannot open file");
    
        // // Write the matrix to the file, row by row
        // for row in &self.robot_pos {
        //     for num in row {
        //         if *num > 0 as usize {
        //             write!(data_file, "1").unwrap();
        //         } else {
        //             write!(data_file, "0").unwrap();
        //         }
                
        //     }
        //     writeln!(data_file).unwrap();
        // }
        // // writeln!(data_file).unwrap();
        // // writeln!(data_file, "-----------------------------------------------------------------{}", iteration).unwrap();

        // // println!("Appended content to a file");

    }

    fn create_image (&mut self, robots: &Vec<Robot> , iteration: usize) {

        self.robot_pos = vec![vec![0; self.len_x]; self.len_y];

        for robot in robots {
            self.robot_pos[robot.curr_pos_y as usize][robot.curr_pos_x as usize] += 1;
        }



        let (width, height) = (self.robot_pos[0].len(), self.robot_pos.len());
        let mut imgbuf = ImageBuffer::new(width as u32, height as u32);

        for (y, row) in self.robot_pos.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                // println!("{} {}", x, pixel);
                let pixel_value: u8 = if *pixel == 0 { 255 } else { 0 };
                // let pixel_value: u8 = 255;
                imgbuf.put_pixel(x as u32, y as u32, Luma([pixel_value]));
            }
        }

        imgbuf.save("output/output_".to_owned()+&iteration.to_string()+".png").unwrap();
    }

    fn show_room (&mut self, robots: &Vec<Robot>) {

        self.place_robots(robots);

        for tile_row in &self.robot_pos {
            println!("{:?}", tile_row);
        }

        println!("\n");

    }

    fn calc_quad_score (&mut self, robots: &Vec<Robot>, iteration: usize) {

        // self.place_robots(robots);

        let mut quad_score: Vec<usize> = vec![0; 4];

        for y in 0..self.len_y/2 {
            for x in 0..self.len_x/2 {
                quad_score[0] += self.robot_pos[y][x];
            }
        }

        for y in 0..self.len_y/2 {
            for x in (self.len_x/2)+1..self.len_x {
                quad_score[1] += self.robot_pos[y][x];
            }
        }

        for y in (self.len_y/2)+1..self.len_y {
            for x in 0..self.len_x/2 {
                quad_score[2] += self.robot_pos[y][x];
            }
        }

        for y in (self.len_y/2)+1..self.len_y {
            for x in (self.len_x/2)+1..self.len_x {
                quad_score[3] += self.robot_pos[y][x];
            }
        }

        // println!("{:?}", quad_score);

        // let mut safety_factor: usize = 1;

        // // for score in quad_score {
        // //     safety_factor *= score;
        // // }

        // // println!("{:?}", safety_factor);

        // for score in quad_score {
        //     if score > robots.len()/2 {
        //         println!("{}", iteration);
        //         // self.show_room(robots);
        //     }
        // }
        

    }

}

fn xy_extractor (raw_test: &str) -> (isize, isize) {

    let x: isize;
    let y: isize;

    let line_split: Vec<&str> = raw_test.split("=").collect();
    x = line_split[1].split(",").collect::<Vec<_>>()[0].parse::<isize>().unwrap();
    y = line_split[1].split(",").collect::<Vec<_>>()[1].parse::<isize>().unwrap();

    return (x, y);
}

fn get_input(file_path: &str, robots: &mut Vec<Robot>) {
    
    let raw_file_contents: String = fs::read_to_string(file_path).unwrap();
    let mut file_contents_iter = raw_file_contents.lines();
    let mut line_str = file_contents_iter.next();
    
    while line_str != None {
        let line: String = line_str.unwrap().to_string();
        let mut line_split = line.split(" ");
        let initial_pos: &str = line_split.next().unwrap();
        let velocity: &str = line_split.next().unwrap();

        let (curr_pos_x, curr_pos_y) = xy_extractor(initial_pos);
        let (velocity_x, velocity_y) = xy_extractor(velocity);

        let robot: Robot = Robot {
            curr_pos_x: curr_pos_x,
            curr_pos_y: curr_pos_y,
            velocity_x: velocity_x,
            velocity_y:velocity_y
        };

        robots.push(robot);

        line_str = file_contents_iter.next();
    }
}

fn restroom_redoubt (room: &mut Room, robots: &mut Vec<Robot>, time_limit: usize) {

    for sec in 1..time_limit {
        for robot in robots.into_iter() {
            robot.move_next(room);
        }

        room.create_image(robots, sec)

        // room.calc_quad_score(robots, sec);
        // room.place_robots(robots, sec);
        // room.show_room(robots);
        // println!("\n");
    }

    // room.show_room(robots);
    

}

fn main() {

    env::set_var("RUST_BACKTRACE", "1");

    let mut robots: Vec<Robot> = vec![];
    let mut room: Room = Room::new(101, 103);

    get_input("input_2.txt", &mut robots);

    // for robot in &robots {
    //     println!("{:?}", robot);
    // };

    // println!("{:?}", room);

    // room.show_room(&robots);

    restroom_redoubt(&mut room, &mut robots, 100000);

    
}
