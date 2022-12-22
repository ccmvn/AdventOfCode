use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;

// Parse eine Eingabestring und erstellt einen Graphen, der aus den Informationen im String generiert wurde
// Der Graphen wird als Vec von Tupeln repräsentiert, wobei jedes Tupel einen Wert und eine Liste von Nachbarn enthält
fn parse_input(regex: &Regex, input: &str) -> Vec<(i32, Vec<i32>)> {
    // Erstelle eine HashMap, die für jeden Knoten eine Liste von Nachbarn enthält
    let mut nodes = HashMap::new();
    nodes.insert("AA".to_string(), 0usize);

    // Initialisiere den Graphen als Vec von Tupeln.
    let mut graph = Vec::<(i32, Vec<usize>)>::new();
    graph.push((0, Vec::new()));

    // Definiert eine Funktion, um einen Knoten in der HashMap zu suchen oder einen neuen Knoten hinzuzufügen
    let mut get_node = |key: &str| {
        let len = nodes.len();
        *nodes.entry(key.to_string()).or_insert(len)
    };

    // Gehe durch jede Zeile und verarbeite sie, wenn sie dem Muster des regulären Ausdrucks entspricht
    for line in input.lines().skip_while(|line| !regex.is_match(line)) {
        let captures = match regex.captures(line) {
            Some(captures) => captures,
            None => continue,
        };

        // Extrahiere den Knoten aus der Zeile und füge ihn zum Graphen hinzu, falls er noch nicht vorhanden ist
        let node = get_node(captures.get(1).unwrap().as_str());

        if node == graph.len() {
            graph.push((
                captures.get(2).unwrap().as_str().parse().unwrap(),
                Vec::new(),
            ));
        } else {
            // Andernfalls aktualisiere den Wert des Knotens
            let node = graph.get_mut(node).unwrap();
            node.0 = captures.get(2).unwrap().as_str().parse().unwrap();
        };

        // Extrahiere die Nachbarn aus der Zeile und füge sie dem Graphen hinzu
        for neighbor in captures.get(3).unwrap().as_str().split(',').map(|str| get_node(str.trim()))
        {
            if neighbor == graph.len() {
                graph.push((0, Vec::new()));
            }
            graph[node].1.push(neighbor);
        }
    }

    // Erstelle eine neue HashMap, um reduzierte Knoten mit ihren Indizes im reduzierten Graphen zu verknüpfen
    let mut reduced_nodes = HashMap::new();
    reduced_nodes.insert(0, 0);

    // Gehe durch jeden Knoten und seinen Wert und Nachbarn im Graphen
    for (node, (value, _)) in graph.iter().enumerate() {
        // Füge den Knoten zur HashMap hinzu, falls er einen Wert hat
        if *value != 0 {
            reduced_nodes.insert(node, reduced_nodes.len());
        }
    }

    // Initialisiere den reduzierten Graphen als Vec von Tupeln
    let mut reduced_graph = vec![(0, vec![0; reduced_nodes.len()]); reduced_nodes.len()];

    // Gehe durch jeden Knoten und seinen Wert und Nachbarn im Graphen
    for (node, (value, _)) in graph.iter().enumerate() {
        let reduced_node;

        // Suche den reduzierten Knoten in der HashMap
        if let Some(v) = reduced_nodes.get(&node) {
            reduced_node = *v;
        } else {
            continue;
        }

        // Setze den Wert des reduzierten Knotens
        reduced_graph[reduced_node].0 = *value;

        // Initialisiere Arrays für den Breitensuche-Algorithmus
        let mut visited = vec![false; nodes.len()];
        let mut queue = VecDeque::new();

        // Markiere den aktuellen Knoten als besucht und füge ihn zur Warteschlange hinzu
        visited[node] = true;
        queue.push_back((node, 1));

        // Führe den Breitensuche-Algorithmus aus, um die Distanzen von jedem Nachbarknoten zu dem aktuellen Knoten zu bestimmen
        while let Some((node, distance)) = queue.pop_front() {
            // Wenn der Nachbarknoten in der HashMap gefunden wurde, aktualisiere seine Distanz im reduzierten Graphen
            if let Some(&next) = reduced_nodes.get(&node) {
                reduced_graph[reduced_node].1[next] = distance;
            }

            // Füge alle Nachbarknoten zur Warteschlange hinzu und markiere sie als besucht
            for &next in graph[node].1.iter() {
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back((next, distance + 1));
                }
            }
        }
    }

    // Gib den reduzierten Graphen zurück
    return reduced_graph;
}

// Führt eine Suche auf dem übergebenen Graphen aus und gibt das Maximum der gefundenen Werte zurück
// Der Graphen wird als Vec von Tupeln repräsentiert, wobei jedes Tupel einen Wert und eine Liste von Nachbarn enthält
fn depth_search(max: i32, graph: &Vec<(i32, Vec<i32>)>, part: usize) -> i32 {
    // Initialisiere ein Array, um zu verfolgen, welche Knoten bereits besucht wurden
    let mut visited = vec![false; graph.len()];

    // Berechne den Gesamtfluss des Graphen
    let mut total_flow = 0;
    for (flow, _) in graph {
        total_flow += flow;
    }

    // Initialisiere eine Variable, um das aktuelle Maximum zu verfolgen
    let mut max_score = 0;

    // Definiert eine rekursive Funktion, um die Suche durchzuführen
    fn implementation(max: i32, graph: &Vec<(i32, Vec<i32>)>, nodes: (usize, usize), distance: (i32, i32), mut current_score: i32, visited: &mut [bool], max_score: &mut i32, max_flow: i32, part: usize) {
        // Aktualisiere den aktuellen Punktestand basierend auf den Werten der aktuellen Knoten und ihren Distanzen
        if part == 0 {
            let (flow0, costs0) = graph.get(nodes.0).unwrap();
            let max_flow = max_flow - flow0;

            current_score += flow0 * (max - distance.0);

            // Aktualisiere das Maximum, falls der aktuelle Punktestand größer ist.
            if current_score > *max_score && max_flow >= 0 {
                *max_score = current_score;
            }

            for (next0, &cost0) in costs0.iter().enumerate() {
                if !visited[next0] && cost0 <= max - distance.0 {
                    // Markiere den Nachbarknoten als besucht
                    visited[next0] = true;

                    // Führe die rekursive Funktion für den Nachbarknoten aus
                    implementation(max, graph, (next0, 0), (distance.0 + cost0, 0), current_score, visited, max_score, max_flow, part);

                    // Markiere den Nachbarknoten als unbesucht, um ihn in späteren Iterationen erneut besuchen zu können
                    visited[next0] = false;
                }
            }
        }
        else {
            let (flow0, costs0) = graph.get(nodes.0).unwrap();
            let (flow1, costs1) = graph.get(nodes.1).unwrap();
            let max_flow = max_flow - flow0 - flow1;

            current_score += flow0 * (max - distance.0);
            current_score += flow1 * (max - distance.1);

            // Aktualisiere das Maximum, falls der aktuelle Punktestand größer ist.
            if current_score > *max_score && max_flow >= 0 {
                *max_score = current_score;
            }

            // Markiere den Nachbarknoten als besucht
            visited[nodes.0] = true;
            visited[nodes.1] = true;

            for (next0, cost0) in costs0.iter().copied().enumerate() {
                let distance0 = distance.0 + cost0;

                if visited[next0] || distance0 > max {
                    continue;
                }

                for (next1, cost1) in costs1.iter().copied().enumerate() {
                    let distance1 = distance.1 + cost1;

                    if next0 == next1 || visited[next1] || distance1 > max {
                        continue;
                    }

                    if max_flow * (max - distance0.min(distance1)) + current_score > *max_score {
                        // Führe die rekursive Funktion für den Nachbarknoten aus
                        implementation(max, graph, (next0, next1), (distance0, distance1), current_score, visited, max_score, max_flow, part);
                    }
                }
            }

            // Markiere den Nachbarknoten als unbesucht, um ihn in späteren Iterationen erneut besuchen zu können
            visited[nodes.0] = false;
            visited[nodes.1] = false;
        }
    }

    visited[0] = true;
    implementation(max, graph, (0, 0), (0, 0), 0, &mut visited, &mut max_score, total_flow, part);

    // Gib den maximalen Score zurück
    return max_score;
}

// Funktion zum auswählen der Aufgabe, als erstes wird der Regex erstellt,
// anschließend wird die Eingabe eingelesen
// Danach wird die Suche auf dem Graphen ausgeführt und das Ergebnis ausgegeben
fn set_part(input: &str, max: i32, part: usize) -> String {
    let regex = Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]*)").unwrap();

    // Parse die Eingabe
    let mut graph = parse_input(&regex, input);

    if part == 1 {
        for node in graph.iter_mut() {
            node.1.push(0);
        }

        graph.push((0, vec![max; graph.len() + 1]));
    }

    // Starte die Suche
    return depth_search(max, &graph, part).to_string();
}

pub fn main () {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Anzahl an Minuten, die maximal für die Suche verwendet werden dürfen
    let max = [30, 26];

    // Gebe die Ergebnisse aus
    for i in 0..2 {
        println!("Most pressure released in {:?} minutes: {}", max[i], set_part(input, max[i], i));
    }
}
