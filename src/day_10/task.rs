// Funktion um die Signalstärke zu ermitteln
fn get_signal_strength(input: &str) -> i32 {
    // Signalstärke
    let mut signal_strength = 0;

    // Temporäre Variablen
    let mut x = 1;
    let mut cycles = 1;
    let mut column = 0;

    // Gehe jede Zeile durch
    for line in input.lines() {
        let mut _op_cycles = 0;
        let mut value = 0;

        // Überprüfe das erste Zeichen
        match line.chars().nth(0).unwrap() {
            'a' => {
                // Füge die Zahl hinzu und erhöhe den Counter
                value = line[5..].parse().unwrap();
                _op_cycles = 2;
            },
            'n' => {
                // Erhöhe den Counter
                _op_cycles = 1;
            },
            _ => panic!("Invalid input"),
        }

        // Gehe solange durch bis der Counter erreicht ist
        for _ in 0.._op_cycles {
            // Wenn die Zahl den cycles matcht erhöhe den Signalstärke
            match cycles {
                20 | 60 | 100 | 140 | 180 | 220 => signal_strength += cycles * x,
                _ => (),
            }

            // Gebe die Sonderzeichen für den Output aus
            if column >= x - 1 && column <= x + 1 {
                print!("\u{2593}");
            } else {
                print!("\u{2591}");
            }

            // Wenn das column die Zahl 39 erreicht, setze eine neue Zeile
            if column == 39 {
                column = 0;
                println!();
            } else {
                column += 1;
            }

            // Erhöhe cycles um 1
            cycles += 1;
        }

        // Erhöhe x um value
        x += value;
    }

    // Gebe die Signalstärke zurück
    return signal_strength;
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Ausgabe der Summe der Signalstärken
    println!("\nSum of the six signal strengths: {}", get_signal_strength(input));
}
