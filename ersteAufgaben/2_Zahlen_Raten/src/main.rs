use std::cmp::Ordering;
use rand::{Rng, thread_rng};
use std::io;

const MAX_TRIES: i32 = 6;

fn main () {
    let number = thread_rng().gen_range(1..=100);
    let mut tries = 0;
    
    println!("Willkommen beim Zahlenraten!");
    
    loop {
        let mut guess = String::new();
        println!("Bitte geben Sie eine Zahl ein. Sie haben noch {} Versuche:", MAX_TRIES - tries);
        io::stdin().read_line(&mut guess).expect("Fehler beim Einlesen der Zahl");
        
        let guessed_number: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\"{}\" ist keine Zahl!", guess.trim());
                continue;
            }
        };
        
        match guessed_number.cmp(&number) {
            Ordering::Less => println!("Die gesuchte Zahl ist größer!"),
            Ordering::Equal => {
                println!("Sie haben die Zahl erraten!");
                break;
            },
            Ordering::Greater => println!("Die gesuchte Zahl ist kleiner!"),
        }
        
        tries += 1;
        
        if tries >= MAX_TRIES {
            println!("Sie haben keine Versuche mehr! Die gesuchte Zahl war: {}", number);
            break;
        }
    }
    
}