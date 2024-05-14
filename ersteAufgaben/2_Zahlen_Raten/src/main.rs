use std::cmp::Ordering;
use rand::{Rng, thread_rng};
use std::io;

fn main () {
    let number = thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();
        println!("Bitte geben Sie eine Zahl ein:");
        io::stdin().read_line(&mut guess).expect("Fehler beim Einlesen der Zahl");
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bitte geben Sie eine Zahl ein!");
                continue;
            }
        };
        
        match guess.cmp(&number) {
            Ordering::Less => println!("Die gesuchte Zahl ist größer!"),
            Ordering::Equal => {
                println!("Sie haben die Zahl erraten!");
                break;
            },
            Ordering::Greater => println!("Die gesuchte Zahl ist kleiner!"),
        }
    }
    
}