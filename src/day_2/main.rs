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

fn main() {
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
    println!("Part 1: {:?}", rounds.iter().map(|x| x.0).sum::<i32>());
    println!("Part 2: {:?}", rounds.iter().map(|x| x.1).sum::<i32>());
}
