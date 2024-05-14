
/*
Aufgabe 1
// Behebe den Fehler mit so wenig Code Modifikationen wie möglich, 
// sodass beim ausführen des Programms Erfolgreich angezeigt wird.
*/

fn main() {
    let x: i32; // Benutzt aber uninitialisiert, ERROR !
    let y: i32; // Nicht benutzt aber auch uninitialisiert, nur eine Warnung !

    assert_eq!(x, 5);
    println!("Erfolgreich!");
}



/*
Aufgabe 2
// Fülle die Lücken aus, sodass der Code kompiliert und erfolgreich ausgeführt wird. 
// Hinweis: Benutze das Schlüsselwort mut, um eine Variable veränderbar zu machen.

//Unkommentiere den folgenden Code, und Kommentiere den Code aus den vergangenen Aufgaben aus.
fn main() {
    let __ __ = 1;
    __ += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}

*/


/*
Aufgabe 3

// Behebe den Fehler unter Benutzung von definiere_x() (darf abgeändert werden)
// Tipp: Benutze String und nicht &str
//Unkommentiere den folgenden Code, und Kommentiere den Code aus den vergangenen Aufgaben aus.

fn main() {
    println!("{}, world", x); 
}

fn definiere_x() {
    let x = "hello";
}
*/