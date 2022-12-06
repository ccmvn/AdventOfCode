// A, X - Stein
// B, Y - Papier
// C, Z - Schere

fn get_round_points(a: char, b: char) -> (i32, i32) {
    match (a, b) {
        ('A', 'X') => (4, 3),
        ('A', 'Y') => (8, 4),
        ('A', 'Z') => (3, 8),
        ('B', 'X') => (1, 1),
        ('B', 'Y') => (5, 5),
        ('B', 'Z') => (9, 9),
        ('C', 'X') => (7, 2),
        ('C', 'Y') => (2, 6),
        ('C', 'Z') => (6, 7),
        _ => panic!("Invalid input"),
    }
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Erstelle einen Vector Tuple
    let mut rounds: Vec<(i32, i32)> = Vec::new();

    // Gehe jede Zeile durch
    for line in input.lines() {
        // Speichere die Punkte der aktuellen Runde
        rounds.push(get_round_points(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()));
    }

    // Gebe die Punkte von Part 1 und Part 2 aus
    for i in 1..3 {
        // Erstelle eine Variable für die Punkte
        let mut points = 0;

        // Gehe jede Runde durch
        for round in rounds.iter() {
            // Addiere die Punkte der Runde zur Gesamtpunktzahl
            points += if i == 1 { round.0 } else { round.1 };
        }

        // Gebe die Punkte aus
        println!("Part {}: {}", i, points);
    }
}
