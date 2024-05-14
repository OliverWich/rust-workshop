# Aufgabe: Actix-Web API

In dieser Aufgabe sollt ihr eine einfache API mit dem Actix-Web Framework in Rust implementieren. Die API soll Informationen über Bücher verwalten und verschiedene Endpunkte bereitstellen.

## Startercode
Bitte nutzt den vorgegebenen [Startercode](https://github.com/Amueller36/rust-workshop-actix-startercode/tree/main)

## Anforderungen

1. Definiert die folgenden Structs:
   - `Book`: Repräsentiert ein Buch mit den Feldern `id` (u32), `title` (String), `author` (String) und `year` (u16).
   - `NewBook`: Repräsentiert die Daten, die beim Erstellen eines neuen Buches übermittelt werden, mit den Feldern `title` (String), `author` (String) und `year` (u16).
   - `ErrorResponse`: Repräsentiert eine Fehlerantwort mit dem Feld `message` (String).

2. Implementiert die folgenden Endpunkte:
   - `GET /books`: Gibt eine Liste aller Bücher zurück.
   - `POST /books`: Erstellt ein neues Buch basierend auf den übermittelten Daten im JSON-Format. Die Daten sollten der Struktur `NewBook` entsprechen.
   - `GET /books/{id}`: Gibt die Details eines bestimmten Buches basierend auf der `id` zurück.

3. Serialisierung und Deserialisierung:
   - Verwendet `serde` und `serde_json`, um die Strukturen `Book` und `NewBook` zu serialisieren und zu deserialisieren.

4. Fehlerbehandlung:
   - Wenn ein Buch mit einer bestimmten `id` nicht gefunden wird, soll der Endpunkt `GET /books/{id}` eine Fehlerantwort mit dem Status 404 (Not Found) und einer entsprechenden Fehlermeldung im JSON-Format zurückgeben.
   - Wenn beim Erstellen eines neuen Buches ungültige Daten übermittelt werden, soll der Endpunkt `POST /books` eine Fehlerantwort mit dem Status 400 (Bad Request) und einer entsprechenden Fehlermeldung im JSON-Format zurückgeben.

5. Verwendet eine `HashMap<u32, Book>` als einfache In-Memory-Datenbank, um die Bücher zu speichern. Die `id` eines Buches soll beim Erstellen automatisch generiert werden.
(Die In Memory Datenbank ist im Started Code bereits gegeben!!)

6. Testet die API mit der vorgegebenen Postman Collection

## Tipps

- Verwendet `actix_web::web::Json` für die Serialisierung und Deserialisierung von JSON.
- Verwendet `actix_web::HttpResponse` für die Rückgabe von Antworten.
- Verwendet `actix_web::web::Path` für das Extrahieren von Pfadparametern wie `id`.
- Ihr müsst wahrscheinlich oft .clone() benutzen!
- Um auf Bücher, bzw. Values einer HashMap zuzugreifen, könnt ihr `.values().clone()` benutzen.

Viel Spaß bei der Implementierung!