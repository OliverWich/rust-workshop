# Rust Zahlen raten Spiel

## Ziel
Lege ein neues Projekt an mit `cargo init zahlen_raten`.
Schreibe ein Programm, das ein einfaches Zahlen raten Spiel implementiert. Der Computer wählt eine zufällige Zahl zwischen 1 und 100 (einschließlich) und der Spieler versucht, diese Zahl zu erraten. Nach jedem Versuch gibt der Computer Hinweise, ob die geratene Zahl zu hoch, zu niedrig oder korrekt ist. Das Spiel endet, wenn der Spieler die Zahl errät.

Diese Übung dient dazu die Grundlegenden Sprachfeatures von Rust in der Praxis umzusetzen.

## Anforderungen
* Das Programm soll eine zufällige Zahl zwischen 1 und 100 (einschließlich) generieren.
* Der Spieler soll aufgefordert werden, eine Zahl zu erraten.
* Das Programm soll die Eingabe des Spielers lesen und darauf reagieren.
* Das Spiel soll so lange fortgesetzt werden, bis der Spieler die korrekte Zahl errät.
* Verwende std::io für die Ein- und Ausgabe.

## Hinweise
Verwende `rand::thread_rng().gen_range()` aus dem Paket `rand` für die Erzeugung von Zufallszahlen. Vergiss nicht, die entsprechenden Abhängigkeiten in der `Cargo.toml`-Datei hinzuzufügen:

```toml
[dependencies]
rand = "0.8.4"
```

Um eine Eingabe des Benutzers einzulesen, importiere `std::io` und benutze `std::io::stdin().read_line()`, :

```rust
use std::io;

let mut guess = String::new();

io::stdin().read_line(&mut guess)
    .expect("Fehler beim Lesen der Eingabe");
```

Für die Konvertierung eines Strings zu einer Zahl kann `mystring.trim().parse()` verwendet werden. (Hinweis, `.parse()` gibt ein Result zurück!)

## Bonus Aufgabe 1
* Falls keine entsprechende Fehlerbehandlung implementiert wurde und der Benutzer eine ungültige Eingabe macht, sollte das Programm eine entsprechende Fehlermeldung ausgeben und den Benutzer erneut zur Eingabe einer Zahl auffordern. Verwende Match Cases.

* Begrenze die Anzahl der Versuche auf 6, die der Benutzer hat, um die Zahl zu erraten.
  
* Nach jedem Zug soll angezeigt werden, wie viele Versuche der Spieler noch übrig hat.

## Bonus Aufgabe 2 - Hierzu gibts jedoch keine Lösung meinerseits.)
* Implementiere eine Option für den Benutzer, das Spiel zu wiederholen.
