use std::collections::HashSet;
use std::collections::HashMap;
use num::complex::Complex;

type Rock = Vec<Complex<i128>>;
type Cavern = HashSet<Complex<i128>>;
type Cache = HashMap<(i128, i128), (i128, i128)>;

// Gibt die Höhe des höchsten Steins im Cavern zurück
fn cavern_height(cavern: &Cavern) -> i128 {
    return cavern.iter().map(|p| p.im).max().unwrap();
}

// Überprüft, ob der Fels an der gegebenen Position hinzugefügt werden kann
// indem überprüft wird, ob alle Punkte des Fels innerhalb der Grenzen des Grids liegen und
// nicht bereits von anderen Steinen im Cavern besetzt sind
fn is_valid_move(cavern: &Cavern, rock: &Rock, pos: Complex<i128>) -> bool {
    rock.iter().all(|c| (c + pos).re >= 0 && (c + pos).re <= 6) && rock.iter().all(|c| !cavern.contains(&(c + pos)))
}

// Fügt einen Fels zum Cavern hinzu, indem der Fels in der gegebenen wind-Reihenfolge horizontal bewegt wird
// Der Fels fällt gerade nach unten, bis er den Boden des Grids erreicht oder auf einem anderen Fels landet,
// an dem Punkt hört er auf zu fallen. Die Funktion gibt den aktualisierten Cavern und den aktualisierten
// Index wi in der wind-Reihenfolge zurück
fn add_rock(cavern: &mut Cavern, rock: &Rock, wind: &Vec<char>, mut wi: i128) -> (Cavern, i128) {
    let mut pos = Complex::new(0, cavern_height(cavern) + 4);
    loop {
        let h_move = if wind[(wi % wind.len() as i128) as usize] == '>' {1} else {-1};
        wi += 1;
        if is_valid_move(cavern, rock, pos + Complex::new(h_move, 0)) {
            pos += Complex::new(h_move, 0);
        }
        if is_valid_move(cavern, rock, pos - Complex::new(0, 1)) {
            pos -= Complex::new(0, 1);
        } else {
            cavern.extend(rock.iter().map(|c| c + pos));
            return (cavern.clone(), wi);
        }
    }
}

// Überprüft den Cache, der eine HashMap ist, die Paare von (i128, i128) auf Paare von (i128, i128)
// abbildet, um zu sehen, ob der aktuelle Zustand des Caverns (dargestellt durch die Argumente i und wi)
// zuvor gesehen wurde. Wenn ja, gibt die Funktion die erwartete Höhe des Caverns nach einer bestimmten
// Anzahl zusätzlicher Steine zurück, basierend auf der Höhenunterschied zwischen dem vorherigen und
// dem aktuellen Zustand und dem Unterschied in der Anzahl der hinzugefügten Steine. Wenn der aktuelle
// Zustand noch nicht gesehen wurde, wird er dem Cache hinzugefügt
fn check_cache(cache: &mut Cache, cavern: &Cavern, rocks: &Vec<Rock>, i: i128, wind: &Vec<char>, wi: i128) -> Option<i128> {
    let height = cavern_height(cavern);
    let key = (i % rocks.len() as i128, wi % wind.len() as i128);
    if let Some((old_i, old_height)) = cache.get(&key) {
        if (1e12 as i128 - i) % (i - old_i) == 0 {
            return Some(height + (1e12 as i128 - i) / (i - old_i) * (height - old_height));
        }
    } else {
        cache.insert(key, (i, height));
    }

    return None;
}

// Hauptfunktion für das Problem. Wenn part true ist, fügt es num Steine zum Cavern hinzu und gibt die endgültige Höhe des Caverns aus
// Wenn part false ist, betritt es eine Schleife, in der es immer wieder Steine zum Cavern hinzufügt, bis es einen Zustand erreicht, der zuvor gesehen wurde
// An diesem Punkt verwendet es den Cache, um die endgültige Höhe des Caverns nach num Steinen zu bestimmen und gibt sie aus
fn get_pressure(rocks: &Vec<Rock>, wind: &Vec<char>, num: i128, part: bool) {
    let mut wi = 0;
    let mut cavern = (0..7).map(|x| Complex::new(x, 0)).collect::<Cavern>();

    if part {
        for i in 0..num {
            let (cavern_, wi_) = add_rock(&mut cavern, &rocks[(i % 5) as usize], wind, wi);
            cavern = cavern_;
            wi = wi_;
        }

        println!("Unit of rocks be after {} rocks have stopped falling: {}", num, cavern_height(&cavern));
    }
    else {
        let mut i = 0;
        let mut cache = Cache::new();

        loop {
            if let Some(answer) = check_cache(&mut cache, &cavern, rocks, i, wind, wi) {
                println!("Unit of rocks be after {} rocks have stopped falling: {}", num, answer);
                break;
            }

            let (cavern_, wi_) = add_rock(&mut cavern, &rocks[(i % 5) as usize], wind, wi);
            cavern = cavern_;
            wi = wi_;
            i += 1;
        }
    }
}

pub fn main() {
    // Lade die input.txt Datei
    let input = include_str!("input.txt");

    // Erstelle die Felsen
    let rocks = vec![
        vec![Complex::new(2, 0), Complex::new(3, 0), Complex::new(4, 0), Complex::new(5, 0)],
        vec![Complex::new(3, 0), Complex::new(2, 1), Complex::new(3, 1), Complex::new(4, 1), Complex::new(3, 2)],
        vec![Complex::new(2, 0), Complex::new(3, 0), Complex::new(4, 0), Complex::new(4, 1), Complex::new(4, 2)],
        vec![Complex::new(2, 0), Complex::new(2, 1), Complex::new(2, 2), Complex::new(2, 3)],
        vec![Complex::new(2, 0), Complex::new(3, 0), Complex::new(2, 1), Complex::new(3, 1)],
    ];

    // Nummern der zugehörigen Aufgaben
    let numbers = [2022, 1_000_000_000_000];

    // Iteriere über die Aufgaben
    for num in numbers.iter() {
        get_pressure(&rocks, &input.chars().collect(), *num, num == &2022);
    }
}
