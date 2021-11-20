use rand::Rng;
use std::io;
use std::process;
use std::string::String;

struct SpeedrunGame {
    target_num: i32,
    range_start: i32,
    range_end: i32,
    num: i32,
    attempts: i32,
    stats: Vec<i32>,
}

impl SpeedrunGame {
    fn speedrun(&mut self) {
        //generates a random number between the start and end values until it hits the targeted value
        while self.num != self.target_num {
            self.num = rand::thread_rng().gen_range(self.range_start..self.range_end);
            self.attempts += 1;
        }
        println!("number, attempts: {}, {}", self.num, self.attempts);
    }
}

fn main() {
    let mut sr: SpeedrunGame = SpeedrunGame {
        target_num: 0,
        range_start: 0,
        range_end: 0,
        num: 0,
        attempts: 0,
        stats: vec![],
    };

    //reads input of user chosen targeted value
    let mut target_num_answer = String::new();
    println!("What number would you like to speedrun?");
    io::stdin()
        .read_line(&mut target_num_answer)
        .expect("Failed to read line");
    let target_num_answer: i32 = target_num_answer
        .trim()
        .parse()
        .expect("Input is not an integer");

    //reads input of user chosen start value
    let mut range_start_answer = String::new();
    println!("What should be the lowest number?");
    io::stdin()
        .read_line(&mut range_start_answer)
        .expect("Failed to read line");
    let range_start_answer: i32 = range_start_answer
        .trim()
        .parse()
        .expect("Input is not an integer");

    //reads input of user chosen end value
    let mut range_end_answer = String::new();
    println!("What should be the highest number?");
    io::stdin()
        .read_line(&mut range_end_answer)
        .expect("Failed to read line");
    let range_end_answer: i32 = range_end_answer
        .trim()
        .parse()
        .expect("Input is not an integer");

    //validates that target value is in range of range_start and range_end
    if target_num_answer > range_start_answer && target_num_answer < range_end_answer {
        sr.target_num = target_num_answer;
        sr.range_start = range_start_answer;
        sr.range_end = range_end_answer;
        recursion(sr);
    } else {
        println!("Please choose a number, that is higher than the lowest number and lower than the highest number.");
    }
}

fn recursion(mut object: SpeedrunGame) {
    object.speedrun();
    println!("Run again? [y/N]: ");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    let answer = answer.trim();

    if answer == "y" {
        recursion(object);
    } else {
        process::exit(1);
    }
}
