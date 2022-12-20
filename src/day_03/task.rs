// Gibt den übergebenen Buchstaben als Priorität zurück
fn get_priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 96,
        'A'..='Z' => c as i32 - 38,
        _ => panic!("Invalid input"),
    }
}

fn set_part(input: &str, part: i32) -> i32 {
    // Summe der Prioritäten
    let mut sum = 0;

    if part == 1 {
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
    }
    else if part == 2 {
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
    }

    // Gebe die Summe zurück
    return sum;
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Gebe die Summe der Prioritäten aus (Part 1 & Part 2)
    for i in 1..3 {
        println!("Part {}: {}", i, set_part(input, i));
    }
}
