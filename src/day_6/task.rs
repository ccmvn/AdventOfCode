use std::collections::HashSet;

// Funktion um den Marker zu ermitteln
fn get_marker_position(stream: &str, position: usize) -> usize {
    for i in position..stream.len() {
        let stream = &stream[i - position..i];
        if stream.chars().collect::<HashSet<char>>().len() == position {
            return i;
        }
    }

    return 0;
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Zeichenlänge um die Markierung zu finden
    let marker = [4, 14];

    // Durchlaufe die Zeichenlängen
    for m in marker.iter() {
        // Ermittle die Position des Markers
        let position = get_marker_position(input, *m);

        // Gib die Position des Markers aus
        println!("Marker (character length {}) was found on position: {}", m, position);
    }
}
