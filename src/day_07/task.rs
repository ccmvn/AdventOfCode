use std::collections::HashMap;

// Funktion um die Verzeichnisgröße zu ermitteln
fn get_directory_size(input: &str) -> HashMap<Vec<String>, i32> {
    let mut directory  = HashMap::new();
    let mut path = Vec::new();

    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["$", "ls"] => {}
            ["$", "cd", ".."] => {
                path.pop();
            }
            ["$", "cd", directory] => {
                path.push(directory.to_string());
            }
            ["dir", directory] => {
                path.push(directory.to_string());
                path.pop();
            }
            [size, _filename] => {
                for i in 1..path.len() + 1 {
                    *directory.entry(path[..i].to_vec()).or_insert(0) += size.parse::<i32>().unwrap();
                }
            }
            _ => {}
        }
    }

    return directory;
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Ermittle die Größe der Verzeichnisse
    let directory = get_directory_size(input);

    // Ermittle die Gesamtgröße der Verzeichnisse aus (unter 100000)
    let mut total_size = 0;
    for (_, size) in directory.iter() {
        if *size <= 100_000 {
            total_size += size;
        }
    }

    // Ausgabe der Gesamtgröße der Verzeichnisse
    println!("Total size of these directories: {}", total_size);

    // Größe des Filesystems
    let filesystem_size = 70_000_000;
    // Benötigter Speicher
    let size_needed = 30_000_000;
    // Ungenutzter Speicher
    let unused_size = filesystem_size - directory.get(&vec!["/".to_string()]).unwrap();
    // Größe die benötigt wird um genügend Speicher zu haben
    let size_to_delete = size_needed - unused_size;

    // Gewünschtes Verzeichnis
    let mut desired_directory = HashMap::new();

    for (k, v) in directory.iter() {
        if *v >= size_to_delete {
            desired_directory.insert(k.to_vec(), *v);
        }
    }

    // Ausgabe der Größe des gewünschten Verzeichnisses
    println!("The size of the desired directory is: {}", desired_directory.values().min().unwrap());
}
