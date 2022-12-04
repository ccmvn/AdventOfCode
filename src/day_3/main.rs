// Gibt den übergebenen Buchstaben als Priorität zurück
fn get_priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 96,
        'A'..='Z' => c as i32 - 38,
        _ => panic!("Invalid input"),
    }
}

fn part1(input: &str) -> i32 {
    // Summe der Prioritäten
    let mut sum = 0;

    // Gehe jede Zeile durch
    for line in input.lines() {
        // Teile die Zeile in zwei Teile und speichere sie in einem Tuple
        let (first, second) = line.split_at(line.len() / 2);

        // Finde die gemeinsamen Buchstaben
        for c in first.chars() {
            if second.contains(c) {
                // Addiere die Priorität des Buchstaben zur Summe
                sum += get_priority(c);
                break;
            }
        }
    }

    // Gebe die Summe zurück
    return sum;
}

fn part2(input: &str) -> i32 {
    // Summe der Prioritäten
    let mut sum = 0;

    // Gehe jede Zeile durch und gebe jede 3 Zeilen aus
    for chunk in input.lines().collect::<Vec<&str>>().chunks(3) {
        // Finde die gemeinsamen Buchstaben
        for c in chunk[0].chars() {
            if chunk[1].contains(c) && chunk[2].contains(c) {
                // Addiere die Priorität des Buchstaben zur Summe
                sum += get_priority(c);
                break;
            }
        }
    }

    // Gebe die Summe zurück
    return sum;
}

fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Gebe die Summe von Part 1 aus
    println!("Part 1: {:?}", part1(input));
    // Gebe die Summe von Part 2 aus
    println!("Part 2: {:?}", part2(input));
}
