use std::collections::HashSet;
use num::complex::Complex;

fn get_rope_way(input: &str) -> Vec<HashSet<Complex<i32>>> {
    // Aktuelle Schlange
    let mut current_rope: Vec<Complex<i32>> = vec![Complex::new(0, 0); 10];
    // Positionen die bereits besucht wurden
    let mut already_seen: Vec<std::collections::HashSet<Complex<i32>>> = current_rope.iter().map(|x| vec![*x].into_iter().collect()).collect();

    // Speichern der Positionen als komplexe Zahlen
    let directions: Vec<Complex<i32>> = vec![Complex::new(1, 0), Complex::new(-1, 0), Complex::new(0, 1), Complex::new(0, -1)];
    let signs = |x: Complex<i32>| Complex::new((x.re > 0) as i32 - (x.re < 0) as i32, (x.im > 0) as i32 - (x.im < 0) as i32);

    for line in input.lines() {
        for _ in 0..line[2..].parse::<i32>().unwrap() {
            // Addiere den Weg den die Schlange gehen muss
            current_rope[0] += directions[match line.chars().nth(0).unwrap() {
                'L' => 0,
                'R' => 1,
                'D' => 2,
                'U' => 3,
                _ => panic!("Invalid input"),
            }];

            for i in 1..10 {
                // Distanz der Schlange
                let distance = current_rope[i - 1] - current_rope[i];

                // Wenn der absolute Wert der Distanz >= 2 ist, füge die Positionen der Schlange hinzu
                if distance.re.abs() >= 2 || distance.im.abs() >= 2 {
                    current_rope[i] += signs(distance);
                    already_seen[i].insert(current_rope[i]);
                }
            }
        }
    }

    // Gebe die Positionen der Schlange zurück
    return already_seen;
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Ermittle den Weg der Schlange
    let rope_way = get_rope_way(input);

    // Ausgabe der Positionen
    println!("The tail of the rope visited at least once by the first snake: {:?}", rope_way[1].len());
    println!("The tail of the rope visited at least once by the second snake: {:?}", rope_way[9].len());
}
