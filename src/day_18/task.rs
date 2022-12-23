use std::collections::HashSet;

// Wandelt eine Zeichenkette mit Zeilen, die durch Kommas getrennte
// Ganzzahlen enthalten, in einen Würfel von Tupeln aus drei i32-Werten um
fn parse_input(input: &str) -> HashSet<(i32, i32, i32)> {
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    // Für jede Zeile in der Eingabe
    for line in input.lines() {
        let line = line.trim();
        // Parse die Zeile in ein Vec von i32
        let coords: Vec<i32> = line
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        // Erstelle ein Tupel aus den Koordinaten
        let coord = (coords[0], coords[1], coords[2]);
        cubes.insert(coord);
    }

    // Gib den Würfel zurück
    return cubes;
}

// Gibt die Koordinaten der sechs Würfel zurück, die an einen gegebenen Würfel angrenzen
fn get_sides(x: i32, y: i32, z: i32) -> HashSet<(i32, i32, i32)> {
    let mut result = HashSet::new();

    // Füge die Koordinaten der sechs benachbarten Würfel hinzu
    result.insert((x + 1, y, z));
    result.insert((x - 1, y, z));
    result.insert((x, y + 1, z));
    result.insert((x, y - 1, z));
    result.insert((x, y, z + 1));
    result.insert((x, y, z - 1));

    // Gib das Ergebnis zurück
    return result;
}

// Berechnet die Anzahl der Würfel in den Seiten jedes Würfels
// in einer gegebenen Menge, die nicht in der Menge enthalten sind
fn surface_area(cubes: &HashSet<(i32, i32, i32)>) -> i32 {
    // Zähle die Anzahl der Würfel in den Seiten
    // jedes Würfels in cubes, die nicht in cubes enthalten sind
    let num_not_in_cubes: i32 = cubes
        .iter()
        .map(|c| get_sides(c.0, c.1, c.2))
        .flatten()
        .filter(|s| !cubes.contains(s))
        .count() as i32;

    // Gib die Anzahl zurück
    return num_not_in_cubes;
}

// Führt eine Breitensuche durch, um alle Würfel zu finden, die von außerhalb
// des gescannten Bereichs erreichbar sind (d.h. jeden Würfel mit einer Koordinate
// von -1) und gibt die Anzahl der Würfel in der gegebenen Menge zurück, die einen
// benachbarten Würfel im Menge der erreichbaren Würfel haben
fn exterior_surface_area(cubes: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut seen: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut todo: Vec<(i32, i32, i32)> = vec![(-1, -1, -1)];

    // Solange es noch Würfel gibt, die untersucht werden müssen
    while !todo.is_empty() {
        // Nimm den nächsten Würfel von der Liste
        let here = todo.pop().unwrap();
        todo.extend(
            get_sides(here.0, here.1, here.2)
                .into_iter()
                .filter(|s| !cubes.contains(s) && !seen.contains(s))
                .filter(|s| s.0 >= -1 && s.0 <= 25 && s.1 >= -1 && s.1 <= 25 && s.2 >= -1 && s.2 <= 25),
        );
        seen.insert(here);
    }

    // Gib die Anzahl der Würfel zurück, die einen benachbarten Würfel im Menge der erreichbaren Würfel haben
    let num_in_seen: i32 = cubes
        .iter()
        .map(|c| get_sides(c.0, c.1, c.2))
        .flatten()
        .filter(|s| seen.contains(s))
        .count() as i32;

    // Gib die Anzahl zurück
    return num_in_seen;
}

pub fn main() {
    // Lese die input.txt ein
    let input = include_str!("input.txt");

    // Parse die Eingabe
    let cubes = parse_input(input);
    // Gib die Anzahl der Würfel in den Seiten jedes Würfels
    let surface_area = surface_area(&cubes);
    println!("Surface area of your scanned lava droplet: {}", surface_area);

    // Gib die Anzahl der Würfel in der gegebenen Menge, die einen
    // benachbarten Würfel im Menge der erreichbaren Würfel haben
    let exterior_surface_area = exterior_surface_area(&cubes);
    println!("Exterior surface area of your scanned lava droplet: {}", exterior_surface_area);
}
