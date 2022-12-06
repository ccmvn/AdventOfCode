#![allow(dead_code)]
use regex::Regex;

// Zum Speichern der zu verschiebenden Elemente
struct Procedure {
    count: usize,
    from: usize,
    to: usize,
}

// Stack für die Stapel
type Stack = Vec<char>;

// Funktion zum parsen der Eingabe
fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    // Regex Objekt erstellen
    let re = Regex::new("((\\[[A-Z]])|( {3})) ?").unwrap();

    // Erstelle einen multi-dimensionalen Vector
    let mut stacks: Vec<Vec<char>> = Vec::new();

    // Gehe die Zeilen rückwärts durch
    for line in input.lines().rev() {
        // Anzazhl der Durchläufe
        let mut count = 0;

        // Gehe die Zeichen mit dem Regex Objekt durch
        for c in re.captures_iter(line) {
            if stacks.len() <= count {
                stacks.push(Stack::new());
            }

            // Zeichen zwischenspeichern
            let c = c[0].chars().nth(1).unwrap();

            // Überprüfe ob das Zeichen Alphabetisch ist
            if c.is_alphabetic() {
                // Füge das Zeichen dem Stapel hinzu
                stacks[count].push(c);
            }

            count += 1;
        }
    }

    return stacks.to_vec();
}

// Funktion um das Ergebnis zu ermitteln
fn get_top_crate(stacks: Vec<Vec<char>>) -> String {
    let mut top_crate = String::new();

    for stack in stacks {
        top_crate.push(*stack.last().unwrap());
    }

    return top_crate;
}

// Funktion um die Elemente zu verschieben
fn set_rearrange_procedure(part: i32, stacks: &mut Vec<Vec<char>>, procedure: &Procedure) -> Vec<Vec<char>> {
    if part == 1 {
        for _ in 0..procedure.count {
            let v = stacks.get_mut(procedure.from - 1).unwrap().pop().unwrap();
            stacks.get_mut(procedure.to - 1).unwrap().push(v);
        }
    }
    else if part == 2 {
        let mut v = Vec::new();

        for _ in 0..procedure.count {
            v.push(stacks.get_mut(procedure.from - 1).unwrap().pop().unwrap());
        }

        for _ in 0..procedure.count {
            stacks.get_mut(procedure.to - 1).unwrap().push(v.pop().unwrap());
        }
    }

    return stacks.to_vec();
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Parse die Stacks
    let stacks: Vec<Vec<char>> = parse_stacks(input);

    // Regex Objekt erstellen
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    // Zum Speichern der Ergebnisse nachdem die Elemente verschoben wurden
    let mut stacks = [stacks.to_vec(), stacks.to_vec()];

    for line in input.lines() {
        // Überprüfe Zeile auf das obige Regex Objekt
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();

            let procedure = Procedure {
                count: caps[1].parse().unwrap(),
                from: caps[2].parse().unwrap(),
                to: caps[3].parse().unwrap(),
            };

            // Neuordnen der Stapel
            for i in 0..2 {
                stacks[i] = set_rearrange_procedure(i as i32 + 1, &mut stacks[i], &procedure);
            }
        }
    }

    // Erhalte die letzten Buchstaben in einer Reihe
    for i in 0..2 {
        println!("Part {}: {}", i + 1, get_top_crate(stacks[i].to_vec()));
    }
}
