#![allow(dead_code)]
use regex::Regex;

// Zum Speichern der zu verschiebenden Elemente
struct Procedure {
    count: usize,
    from: usize,
    to: usize,
}

// Funktion um das Ergebnis zu ermitteln
fn get_top_crate(stacks: Vec<Vec<char>>) -> String {
    let mut top_crate = String::new();

    for stack in stacks {
        top_crate.push(*stack.last().unwrap());
    }

    return top_crate;
}

// Funktion um die Elemente zu verschieben (Part 1)
fn set_rearrange_procedure_part1(stacks: &mut Vec<Vec<char>>, procedure: &Procedure) -> Vec<Vec<char>> {
    for _ in 0..procedure.count {
        let v = stacks.get_mut(procedure.from - 1).unwrap().pop().unwrap();
        stacks.get_mut(procedure.to - 1).unwrap().push(v);
    }

    return stacks.to_vec();
}

// Funktion um die Elemente zu verschieben (Part 2)
fn set_rearrange_procedure_part2(stacks: &mut Vec<Vec<char>>, procedure: &Procedure) -> Vec<Vec<char>> {
    let mut v = Vec::new();

    for _ in 0..procedure.count {
        v.push(stacks.get_mut(procedure.from - 1).unwrap().pop().unwrap());
    }

    for _ in 0..procedure.count {
        stacks.get_mut(procedure.to - 1).unwrap().push(v.pop().unwrap());
    }

    return stacks.to_vec();
}

pub fn main() {
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(9);
    stacks.push("TDWZVP".chars().collect());
    stacks.push("LSWVFJD".chars().collect());
    stacks.push("ZMLSVTBH".chars().collect());
    stacks.push("RSJ".chars().collect());
    stacks.push("CZBGFMLW".chars().collect());
    stacks.push("QWVHZRGB".chars().collect());
    stacks.push("VJPCBDN".chars().collect());
    stacks.push("PTBQ".chars().collect());
    stacks.push("HGZRC".chars().collect());

    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Regex Objekt erstellen
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    // Zum Speichern der Ergebnisse nachdem die Elemente verschoben wurden
    let mut stacks_part1 = stacks.to_vec();
    let mut stacks_part2 = stacks.to_vec();

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
            stacks_part1 = set_rearrange_procedure_part1(&mut stacks_part1, &procedure);
            stacks_part2 = set_rearrange_procedure_part2(&mut stacks_part2, &procedure);
        }
    }

    // Erhalte die letzten Buchstaben in einer Reihe
    println!("Top crate from task 1: {}", get_top_crate(stacks_part1));
    println!("Top crate from task 2: {}", get_top_crate(stacks_part2));
}
