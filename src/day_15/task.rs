use std::cmp::{max, min};
use regex::Regex;

// Sensorstruktur
struct Sensor {
    sensor_pos: (i32, i32), // Die x- und y-Position des Sensors
    nearest_beacon_pos: (i32, i32), // Die x- und y-Position des nächsten Beacons
    sensor_radius: u32, // Der Radius des Sensors
}

// Parst den Input-String und erstellt eine Liste von Sensoren
fn parse_input(input: &str) -> Vec<Sensor> {
    // Erstelle einen RegEx, um die x- und y-Positionen des Sensors und des Beacons aus dem Input-String zu extrahieren
    let re: Regex = Regex::new(r"x=(-?\d+), y=(-?\d+).*x=(-?\d+), y=(-?\d+)").unwrap();

    // Iteriere über alle Treffer des RegEx und erstelle für jeden Treffer einen Sensor
    re.captures_iter(input)
        .map(|cap| {
            // Hole die x- und y-Positionen des Sensors und Beacons aus dem Treffer
            let sensor_x: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let sensor_y: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
            let beacon_x: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
            let beacon_y: i32 = cap.get(4).unwrap().as_str().parse().unwrap();

            // Erstelle einen Sensor mit den gesammelten Informationen
            Sensor {
                sensor_pos: (sensor_x, sensor_y),
                nearest_beacon_pos: (beacon_x, beacon_y),
                sensor_radius: sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y),
            }
        })
        .collect() // Sammle alle erstellten Sensoren in einer Liste
}

// Berechnet die Grenzen (minimaler x-Wert, maximaler x-Wert, minimal y-Wert, maximaler y-Wert)
// für die Punkte, die von den Sensoren abgedeckt werden
fn calculate_bounds(sensors: &[Sensor]) -> ((i32, i32), (i32, i32)) {
    // Iteriere über alle Sensoren und berechne die Grenzen, die von allen Sensoren abgedeckt werden
    sensors.iter().fold(((0, 0), (0, 0)), |bounds, sensor| {
        let min_x = sensor.sensor_pos.0 - sensor.sensor_radius as i32; // Minimaler x-Wert für diesen Sensor
        let max_x = sensor.sensor_pos.0 + sensor.sensor_radius as i32; // Maximaler x-Wert für diesen Sensor
        let min_y = sensor.sensor_pos.1 - sensor.sensor_radius as i32; // Minimaler y-Wert für diesen Sensor
        let max_y = sensor.sensor_pos.1 + sensor.sensor_radius as i32; // Maximaler y-Wert für diesen Sensor

        // Aktualisiere die Grenzen mit den Grenzen für diesen Sensor
        ((
             min(bounds.0.0, min_x),
             max(bounds.0.1, max_x),
         ), (
             min(bounds.1.0, min_y),
             max(bounds.1.1, max_y),
         ))
    })
}

// Prüft, ob der angegebene Punkt eine Lösung ist (d.h. ob er außerhalb der Abdeckung aller Sensoren liegt)
fn is_solution(point: (i32, i32), sensors: &[Sensor]) -> bool {
    // Iteriere über alle Sensoren
    for sensor in sensors {
        // Berechne die Distanz zwischen dem Punkt und dem Sensor
        let distance = (point.0 - sensor.sensor_pos.0).abs() + (point.1 - sensor.sensor_pos.1).abs();
        // Wenn die Distanz kleiner oder gleich dem Radius des Sensors ist
        if distance <= sensor.sensor_radius as i32 {
            return false;
        }
    }

    // Wenn der Punkt außerhalb der Abdeckung aller Sensoren liegt
    return true;
}

fn set_part(line_num: i32, sensors: &[Sensor], part: bool) {
    if part {
        let (x_bounds, y_bounds) = calculate_bounds(sensors); // Berechne die Grenzen für die Punkte, die von den Sensoren abgedeckt werden
        let width = x_bounds.0.abs_diff(x_bounds.1); // Berechne die Breite der Fläche, die von den Sensoren abgedeckt wird

        let mut line = vec![true; width as usize]; // Erstelle eine Liste mit der Breite der Fläche, die von den Sensoren abgedeckt wird
        let mut num_no_beacon = 0; // Die Anzahl der Punkte, die von den Sensoren abgedeckt werden

        if line_num >= y_bounds.0 && line_num <= y_bounds.1 { // Wenn die angegebene Zeile innerhalb der Grenzen liegt
            for sensor in sensors { // Iteriere über alle Sensoren
                let mut current_x = x_bounds.0; // Der aktuelle x-Wert

                for pos in &mut line { // Iteriere über alle Positionen in der Zeile
                    if *pos { // Wenn die Position noch nicht abgedeckt wurde
                        let distance = (sensor.sensor_pos.0 - current_x).abs() + (sensor.sensor_pos.1 - line_num).abs(); // Berechne die Distanz zwischen dem Punkt und dem Sensor
                        if distance <= sensor.sensor_radius as i32 { // Wenn der Punkt innerhalb der Abdeckung des Sensors liegt
                            *pos = false; // Markiere die Position als abgedeckt
                        }
                    }

                    current_x += 1; // Erhöhe den x-Wert um 1
                }
            }

            for sensor in sensors { // Iteriere über alle Sensoren
                if sensor.nearest_beacon_pos.1 == line_num { // Wenn der Beacon in der angegebenen Zeile liegt
                    line[(sensor.nearest_beacon_pos.0 - x_bounds.0) as usize] = true; // Markiere die Position des Beacons als nicht abgedeckt
                }
            }

            num_no_beacon += line.iter().filter(|x| !*x).count(); // Zähle die Anzahl der nicht abgedeckten Positionen
        }

        println!("Positions cannot contain a beacon: {}", num_no_beacon);
    }
    else {
        let mut found_solution: (i32, i32) = (-1, -1); // Die Position der Lösung

        for sensor in sensors { // Iteriere über alle Sensoren
            let mut y = sensor.sensor_pos.1 - sensor.sensor_radius as i32 - 1; // Der aktuelle y-Wert
            for x in sensor.sensor_pos.0..=(sensor.sensor_pos.0 + sensor.sensor_radius as i32 + 1) {
                if is_solution((x, y), sensors) && x > 0 && y > 0 && x < line_num && y < line_num {
                    found_solution = (x, y); // Speichere die Position der Lösung
                    break;
                }
                y -= 1; // Verringere den y-Wert um 1

                if is_solution((x, y), sensors) && x > 0 && y > 0 && x < line_num && y < line_num {
                    found_solution = (x, y); // Speichere die Position der Lösung
                    break;
                }
                y += 2; // Erhöhe den y-Wert um 2
            }

            let mut y = sensor.sensor_pos.1;
            for x in (sensor.sensor_pos.0 - sensor.sensor_radius as i32 - 1)..=sensor.sensor_pos.0 {
                if is_solution((x, y), sensors) && x > 0 && y > 0 && x < line_num && y < line_num {
                    found_solution = (x, y); // Speichere die Position der Lösung
                    break;
                }
                y += 1; // Erhöhe den y-Wert um 1

                if is_solution((x, y), sensors) && x > 0 && y > 0 && x < line_num && y < line_num {
                    found_solution = (x, y); // Speichere die Position der Lösung
                    break;
                }
                y -= 2; // Verringere den y-Wert um 2
            }
        }

        println!("Tuning frequency: {}", found_solution.0 as u64 * 4_000_000 + found_solution.1 as u64);
    }
}

pub fn main() {
    let line_num = [2000000, 4000000];

    // Lese die input.txt Datei
    let input = include_str!("input.txt");
    // Erstelle einen Vector mit allen Sensoren
    let sensors = parse_input(input);

    // Gebe die Lösung für Part 1 und Part 2 aus
    for i in 0..2 {
        set_part(line_num[i], &sensors, i == 0);
    }
}
