use rand::Rng;
use std::io;
use std::string::String;

fn main() {
    game_setup()
}

fn game_setup() {
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
        speedrun(target_num_answer, range_start_answer, range_end_answer);
    } else {
        println!("Please choose a number, that is higher than the lowest number and lower than the highest number.");
    }
}

fn speedrun(target_num: i32, range_start: i32, range_end: i32) {
    let mut num: i32 = 0;
    let mut attempts: i32 = 0;

    //generates a random number between the start and end values until it hits the targeted value
    while num != target_num {
        num = rand::thread_rng().gen_range(range_start..range_end);
        attempts += 1;
    }
    println!("number, attempts: {}, {}", num, attempts);

    //If user types in "y" the script calls itself with the same values
    println!("Run again? [y/N]: ");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    let answer = answer.trim();

    if answer == "y" {
        speedrun(target_num, range_start, range_end);
    } else {
        game_setup();
    }
}
