use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Willkommen zum Zahlenraten Spiel!");
    
    let zu_erratende_zahl = rand::thread_rng().gen_range(1..=100);
    let mut versuche = 0;
    let max_versuche = 6;

    loop {
        println!("Bitte errate eine Zahl (noch {} Versuche übrig):", max_versuche - versuche);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Fehler beim Lesen der Eingabe");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Das war keine gültige Zahl. Bitte versuche es erneut.");
                continue;
            }
        };

        versuche += 1;

        match guess.cmp(&zu_erratende_zahl) {
            Ordering::Less => println!("Zu klein!"),
            Ordering::Greater => println!("Zu groß!"),
            Ordering::Equal => {
                println!("Richtig! Du hast die Zahl in {} Versuchen erraten.", versuche);
                break;
            }
        }

        if versuche == max_versuche {
            println!("Du hast alle Versuche aufgebraucht. Die gesuchte Zahl war {}.", zu_erratende_zahl);
            break;
        }
    }
}
