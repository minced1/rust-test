use std::io;
use rand::Rng;

fn main() {
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

    speedrun(target_num_answer, range_start_answer, range_end_answer)
}

fn speedrun(target_num: i32, range_start: i32, range_end: i32) {
    let mut num: i32 = 0;
    let mut attempts: i32 = 0;

    //generates a random number between the start and end values until it hits the targeted value
    while num != target_num {
       num = rand::thread_rng().gen_range(range_start..range_end);
       attempts = attempts+1;
    }
    println!("number, attempts: {}, {}", num, attempts);

    //If user types in "y" the script calls itself with the same values
    println!("Run again? y/N");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    let answer = answer.trim();

    if answer == "y" {
        speedrun(target_num, range_start, range_end);
    }
}