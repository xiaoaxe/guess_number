use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    guess_number();
}

fn guess_number(){
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Fail to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("err: {}", err);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!\n"),
            Ordering::Greater => println!("Too Big!\n"),
            Ordering::Equal => {
                println!("You Win!\n Bye Bye!");
                break;
            }
        }
    }
}
