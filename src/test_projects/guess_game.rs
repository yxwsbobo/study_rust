use rand::Rng;
use std::io;
use std::cmp::Ordering;


pub fn start(){
    println!("in start guess game");

    let right_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("please input a number:");

        let mut user_number = String::new();
        io::stdin().read_line(&mut user_number).expect("read line error");

        let user_number:u32 = match user_number.trim().parse(){
            Result::Ok(num) => num,
            Result::Err(_) => {
                println!("{} is not a number!", user_number.trim());
                continue;
            }
        };

        println!("your guess number is: {}", user_number);

        match right_number.cmp(&user_number) {
            Ordering::Greater => println!("your number is too less"),
            Ordering::Less => println!("your number is too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }

    println!("game over");
}