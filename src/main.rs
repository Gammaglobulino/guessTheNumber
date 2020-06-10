use rand::Rng;
use std::io::stdin;

fn main() {
    let number =rand::thread_rng().gen_range(1,101);
    println!("Guess the number game!!");
    loop{
        println!("Enter your guess:>>");
        let mut buffer=String::new();
        match stdin().read_line(&mut buffer){
            Ok(_) =>{
                let parsed=buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess <1 || guess > 100{
                            println!("Your guess is out of range");
                        }else if guess < number{
                            println!("Your guess is low");
                        }else if guess > number {
                            println!("Your guess is high")
                        }else {
                            println!("Your number {} is correct! Well done!",guess);
                            break;
                        }
                    },
                    Err(err) => {
                        println!("Cuold not read from the input:{}, try again!", err);
                    }
                }

            },
            Err(_) => continue,
        }
    }
}
