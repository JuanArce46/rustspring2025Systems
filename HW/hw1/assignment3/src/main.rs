use std::io;

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 7; // Hard-coded secret number
    let mut attempts = 0;
    
    loop {
        println!("Enter your guess: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        attempts += 1;

        let result = check_guess(guess, secret);
        
        if result == 0 {
            println!("{} is correct!", guess);
            break;
        } else if result == 1 {
            println!("{} is too high!", guess);
        } else {
            println!("{} is too low!", guess);
        }
    }
    
    println!("It took {} attempts to guess correctly.", attempts);
}
