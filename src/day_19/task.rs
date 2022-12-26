use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type BluePrint = (u32, u32, u32, u32, u32, u32, u32); // Typ alias für ein Tupel mit 7 u32-Werten

// Struktur für den aktuellen Zustand von Ressourcen und Robotern
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct State {
    time: u32, // Zeit

    ore: u32, // Erz
    ore_bot: u32, // Anzahl der Erz-Bot

    clay: u32, // Ton
    clay_bot: u32, // Anzahl der Ton-Bot

    obsidian: u32, // Obsidian
    obsidian_bot: u32, // Anzahl der Obsidian-Bot

    geode: u32, // Geode
    geode_bot: u32, // Anzahl der Geode-Bot
}

// Default-Implementierung für den State-Typ
impl Default for State {
    fn default() -> Self { // Standard-Werte für einen neuen State festlegen
        Self {
            time: 0,
            ore: 0,
            ore_bot: 1,
            clay: 0,
            clay_bot: 0,
            obsidian: 0,
            obsidian_bot: 0,
            geode: 0,
            geode_bot: 0,
        }
    }
}

// Funktion zum Aktualisieren der Ressourcen
fn update_resources(current_state: &mut State) {
    current_state.ore += current_state.ore_bot; // Erzmenge erhöhen
    current_state.clay += current_state.clay_bot; // Tonmenge erhöhen
    current_state.geode += current_state.geode_bot; // Geodemenge erhöhen
    current_state.obsidian += current_state.obsidian_bot; // Obsidianmenge erhöhen
}

// Funktion zum Erstellen von neuen Zuständen durch Hinzufügen von Robotern
fn update_bots(current_state: State, blue_print: &BluePrint, to_visit: &mut VecDeque<State>, n_ge: u32, n_ob: u32, n_cl: u32, n_or: u32) {
    if n_ge == 1 { // Wenn ein Geode-Bot hinzugefügt werden kann
        let mut new_state = current_state.clone();
        new_state.geode_bot += 1; // Anzahl der Geode-Bot um 1 erhöhen
        new_state.ore -= blue_print.5; // Erzmenge entsprechend reduzieren
        new_state.obsidian -= blue_print.6; // Obsidianmenge entsprechend reduzieren

        to_visit.push_back(new_state);
        return;
    }

    if n_ob == 1 && current_state.obsidian_bot < blue_print.6 { // Wenn ein Obsidian-Bot hinzugefügt werden kann
        let mut new_state = current_state.clone();
        new_state.obsidian_bot += 1; // Anzahl der Obsidian-Bot um 1
        new_state.ore -= blue_print.3; // Erzmenge entsprechend reduzieren
        new_state.clay -= blue_print.4; // Tonmenge entsprechend reduzieren

        to_visit.push_back(new_state);
    }

    if n_cl == 1 && current_state.clay_bot < blue_print.4 { // Wenn ein Ton-Bot hinzugefügt werden kann
        let mut new_state = current_state.clone();
        new_state.clay_bot += 1; // Anzahl der Ton-Bot um 1 erhöhen
        new_state.ore -= blue_print.2; // Erzmenge entsprechend reduzieren

        to_visit.push_back(new_state);
    }

    // Wenn ein Erz-Bot hinzugefügt werden kann
    if n_or == 1 && current_state.ore_bot < *[blue_print.1, blue_print.2, blue_print.3, blue_print.5].iter().max().unwrap() {
        let mut new_state = current_state.clone();
        new_state.ore_bot += 1; // Anzahl der Erz-Bot um 1 erhöhen
        new_state.ore -= blue_print.1; // Erzmenge entsprechend reduzieren

        to_visit.push_back(new_state);
    }

    to_visit.push_back( current_state);
}

// Funktion zum Verarbeiten eines Zustands
fn visit_state(mut current_state: State, to_visit: &mut VecDeque<State>, seen: &mut HashSet<State>, blue_print: &BluePrint, best_geode: &mut HashMap<u32, u32>, limit: u32, possible_scores: &mut Vec<u32>) {
    // Wenn der Zustand schon einmal besucht wurde
    if seen.contains(&current_state) {
        return;
    }

    seen.insert(current_state.clone());

    current_state.time += 1; // Zeit um 1 erhöhen

    if current_state.time > limit { // Wenn die Zeitbegrenzung erreicht wurde
        // Möglichen Score hinzufügen (Anzahl der Geode-Bot * Wert einer Geode)
        possible_scores.push(blue_print.0 as u32 * current_state.geode as u32);
        return;
    }

    // Anzahl der möglichen Roboter initialisieren
    let (mut n_cl, mut n_or, mut n_ob, mut n_ge) = (0, 0, 0, 0);

    // Wenn genügend Erz vorhanden ist, um einen Erz-Bot zu bauen
    if current_state.ore >= blue_print.1 {
        n_or += 1;
    }

    // Wenn genügend Erz und Ton vorhanden sind, um einen Ton-Bot zu bauen
    if current_state.ore >= blue_print.2 {
        n_cl += 1;
    }

    // Wenn genügend Erz, Ton und Obsidian vorhanden sind, um einen Obsidian-Bot zu bauen
    if current_state.ore >= blue_print.3 && current_state.clay >= blue_print.4 {
        n_ob = 1;
    }

    // Wenn genügend Erz und Obsidian vorhanden sind, um einen Geode-Bot zu bauen
    if current_state.ore >= blue_print.5 && current_state.obsidian >= blue_print.6 {
        n_ge = 1;
    }

    update_resources(&mut current_state);

    let delta = if limit != 24 && current_state.time > 21 {
        1
    } else {
        0
    };

    if current_state.geode + delta < *best_geode.entry(current_state.time).or_insert(0) {
        return;
    } else if current_state.geode > *best_geode.entry(current_state.time).or_insert(0) {
        *best_geode.get_mut(&current_state.time).unwrap() = current_state.geode;
    }

    update_bots(current_state, blue_print, to_visit, n_ge, n_ob, n_cl, n_or);
}

// Funktion zum Berechnen der besten Lösung
fn get_max_geode(blue_print: &BluePrint, limit: u32) -> u32 {
    let init_state = State {
        ..Default::default()
    };
    let mut possible_scores: Vec<u32> = Vec::new();

    let mut to_visit: VecDeque<State> = VecDeque::new();
    to_visit.push_back(init_state);

    let mut seen: HashSet<State> = HashSet::new();

    let mut best_geode: HashMap<u32, u32> = HashMap::new();

    while !to_visit.is_empty() {
        let current_state = to_visit.pop_front().unwrap();
        visit_state(current_state, &mut to_visit, &mut seen, &blue_print, &mut best_geode, limit, &mut possible_scores);
    }

    return *possible_scores.iter().max().unwrap_or(&0);
}

// Funktion zum Auslesen der Eingabedatei
fn parse_input(input: &str) -> Vec<BluePrint> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut blue_prints: Vec<BluePrint> = Vec::new();

    for l in input.lines() {
        let p = re
            .find_iter(l)
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        blue_prints.push((p[0], p[1], p[2], p[3], p[4], p[5], p[6]));
    }

    return blue_prints;
}

// Funktion zur Ausgabe des Ergebnisses
fn get_solution(blue_prints: &Vec<BluePrint>) {
    // Ergebnis von Part 1
    println!("Quality level of all of the blueprints: {}", blue_prints.iter().map(|x| get_max_geode(x, 24) as u32).sum::<u32>());
    // Ergebnis von Part 2
    println!("Multiplied numbers: {}", blue_prints.iter().take(3).map(|x| get_max_geode(x, 32))
        .reduce(|x, y| x * y).unwrap() / blue_prints.iter().take(3).map(|x| x.0)
        .reduce(|x, y| x * y).unwrap()
    );
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");
    // Erstelle ein Vector mit den Blueprints
    let blue_prints = parse_input(input);
    // Gebe das Ergebnis aus
    get_solution(&blue_prints);
}
