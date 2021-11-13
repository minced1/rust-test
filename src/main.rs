use std::io;
use rand::Rng;

fn main() {
    speedrun();
    println!("Run again? y/N");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    let answer = answer.trim();

    if answer == "y" {
        main();
    }
}

fn speedrun() {
    let mut num: i32 = 0;
    let mut attempts: i32 = 0;

    while num != 69420 {
       num = rand::thread_rng().gen_range(10000..99999);
       attempts = attempts+1;
    }
    println!("number, attempts: {}, {}", num, attempts);
}