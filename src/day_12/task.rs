// Diese Funktion erstellt eine Karte aus einem Eingabestring
fn get_map(input: &str) -> Vec<u8> {
    input.bytes()
        .filter(|b| b != &b'\n') // Nur Bytes ohne Zeilenumbrüche werden berücksichtigt
        .map(|b| b.to_ascii_lowercase() - b'a')// Jedes Zeichen wird in eine Zahl umgewandelt
        .collect() // Alle Bytes werden in einem Vector gespeichert
}

// Diese Funktion bestimmt die Dimensionen der Karte
fn get_dimensions(input: &str) -> (usize, usize) {
    let w = input.bytes().position(|b| b == b'\n').unwrap(); // Ermittle die Breite der Karte
    let h = input.len() / w; // Ermittle die Höhe der Karte

    return (w, h);
}

// Diese Funktion bestimmt die Start- und Endposition
fn get_start_and_end(input: &str) -> (usize, usize) {
    let start = input.bytes().position(|b| b == b'S').unwrap(); // Ermittle die Startposition
    let end = input.bytes().position(|b| b == b'E').unwrap(); // Ermittle die Endposition

    return (start, end);
}

// Diese Funktion bestimmt die benachbarten Zellen einer Positio
fn get_adjacent_cells(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut adjacents = Vec::new();

    if x > 0 {
        adjacents.push((x - 1, y)); // füge die Position darüber hinzu
    }
    if y > 0 {
        adjacents.push((x, y - 1)); // füge die Position links hinzu
    }
    if x < w - 1 {
        adjacents.push((x + 1, y)); // füge die Position rechts hinzu.
    }
    if y < h - 1 {
        adjacents.push((x, y + 1)); // füge die Position darunter hinzu.
    }

    return adjacents;
}

// Diese Funktion gibt die Ergebnisse aus
fn print_results(w: usize, h: usize, start: usize, end: usize, map: Vec<u8>) {
    let mut path_length;

    path_length = pathfinding::directed::bfs::bfs(
        &(end % w, end / w), // Beginne mit der Endposition
        |(x, y)| {
            let cur = map[y * w + x]; // Ermittle den Wert
            get_adjacent_cells(*x, *y, w, h) // Ermittle alle benachbarten Positionen
                .iter()
                // Filter alle Positionen, deren Wert kleiner als der aktuelle Wert ist
                .filter(|(x, y)| map[y * w + x] >= cur.saturating_sub(1))
                .cloned() // Kopiere alle Positionen
                .collect::<Vec<_>>() // Speichere die Positionen in einem Vector
        },
        |&p| p == (start % w, start / w), // Stoppe, wenn die Startposition erreicht wurde
    ).unwrap().len() - 1; // Subtrahiere die Anzahl der Schritte um 1, da die Endposition ein Schritt war

    println!("Fewest steps required to move from your current position to the location with the best signal: {}", path_length);

    path_length = map.iter()
        .enumerate()
        .filter(|(_, b)| **b == 0) // Filter alle Positionen mit Wert 0
        .filter_map(|(start, _)| pathfinding::directed::bfs::bfs(
            &(start % w, start / w), // Beginne mit der Startposition
            |(x, y)| {
                let cur = map[y * w + x];
                get_adjacent_cells(*x, *y, w, h) // Ermittle alle benachbarten Positionen
                    .iter()
                    // Filter alle Positionen, deren Wert höher als der aktuelle Wert ist
                    .filter(|(x, y)| map[y * w + x] <= cur + 1)
                    .cloned() // Kopiere alle Positionen
                    .collect::<Vec<_>>() // Speichere die Positionen in einem Vector
            },
            |&p| p == (end % w, end / w), // Stoppe, wenn die Endposition erreicht wurde
        ).map(|r| r.len() - 1)).min().unwrap(); // Ermittle die Anzahl der Schritte und nehme das Minimum

    println!("Fewest steps required to move starting from any square with elevation a to the location with the best signal: {}", path_length);
}

pub fn main() {
    let input = include_str!("input.txt");

    let mut map = get_map(input);
    let (w, h) = get_dimensions(input);
    let (mut start, mut end) = get_start_and_end(input);
    // Setze den Wert der Start- und Endposition auf 0 und 25
    (start, end, map[start], map[end]) = (start - start / (w + 1), end - end / (w + 1), 0, 25);

    print_results(w, h, start, end, map); // Gib die Ergebnisse aus
}
