use std::cmp::Ordering;
use rand::{Rng, thread_rng};
use std::io;

const MAX_TRIES: i32 = 6;

fn main () {
    println!("Willkommen beim Zahlenraten!");

    loop {
        game_loop();

        if !ask_for_retry() {
            println!("Auf Wiedersehen!");
            break;
        }
    }
}

fn ask_for_retry() -> bool {
    loop {
        println!("Möchten Sie nochmal spielen? (J/n)");

        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Fehler beim Einlesen der Antwort");

        let raw_command = play_again.trim().to_lowercase();

        let command = if raw_command.is_empty() { "j" } else { &raw_command };

        match command {
            "j" => return true,
            "n" => return false,
            _ => {
                println!("Ungültige Eingabe! Bitte geben Sie \"j\" oder \"n\" ein.");
                continue;
            }
        }
    }
}

fn game_loop() {
    let number = thread_rng().gen_range(1..=100);
    let mut tries = 0;

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
