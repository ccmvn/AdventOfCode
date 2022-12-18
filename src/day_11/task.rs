use itertools::Itertools;

// Enum zum Definieren von Operationen
enum OperationType {
    Square,
    Add(usize),
    Multiply(usize),
}

// Struktur für eine Operation
struct Operation {
    op: OperationType,
}

// Struktur für einen Affen
struct Monkey {
    bag: Vec<usize>,
    op: Operation,
    divisible: usize,
    trueTarget: usize,
    falseTarget: usize,
    test: usize,
}

// Eingabe parsen
fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n") // Eingabe in Abschnitte splitten
        .map(|m| {
            // Zeilen in Abschnitt in Elemente splitten
            let l: Vec<_> = m.lines().map(|l| l.split(": ").last().unwrap()).collect();
            // Operation aus Element 3 extrahieren
            let op: Vec<_> = l[2].rsplit_once("= ").unwrap().1.split(' ').collect();

            // Operation aus Element 3 in Operationstruktur umwandeln
            let operation = match op[2] {
                "old" => Operation { op: OperationType::Square },
                b => match (op[1], b.parse::<usize>().unwrap()) {
                    ("+", n) => Operation { op: OperationType::Add(n) },
                    ("*", n) => Operation { op: OperationType::Multiply(n) },
                    _ => unreachable!(),
                },
            };
            // Struktur erstellen
            Monkey {
                bag: l[1].split(", ").map(|n| n.parse().unwrap()).collect(),
                op: operation,
                divisible: l[3].rsplit_once(' ').unwrap().1.parse().unwrap(),
                trueTarget: l[4].rsplit_once(' ').unwrap().1.parse().unwrap(),
                falseTarget: l[5].rsplit_once(' ').unwrap().1.parse().unwrap(),
                test: 0,
            }
        })
        .collect()
}

// Funktion zum erstellen der Simulation
fn setup_simulation(m: &Vec<Monkey>) -> (usize, Vec<Vec<usize>>) {
    let mo = m.iter().map(|m| m.divisible).product();
    let bags = vec![vec![]; m.len()];

    return (mo, bags);
}

// Funktion zum simulieren der Runden
fn simulate_round(m: &mut Vec<Monkey>, bags: &mut Vec<Vec<usize>>, mo: usize, rounds: impl Iterator<Item=usize>, condition: bool) {
    // Führe für jede Runde aus
    rounds.for_each(|_| {
        // Iteriere über jeden Affen und dessen Index
        m.iter_mut().enumerate().for_each(|(i, m)| {
            m.bag.append(&mut bags[i]);
            m.bag.drain(0..).for_each(|mut n| {
                // Führe die Operation aus
                n = match m.op.op {
                    OperationType::Square => n * n,
                    OperationType::Add(x) => n + x,
                    OperationType::Multiply(x) => n * x,
                };
                // Wähle eine Bedingung
                n = match condition {
                    true => n / 3 % mo,
                    false => n % mo,
                };

                bags[if n % m.divisible == 0 { m.trueTarget } else { m.falseTarget }].push(n);

                // Erhöhe die Anzahl inspizierten Items
                m.test += 1;
            });
        });
    });
}

// Funktion um das Ergebnis der Simulation zu erhalten
fn get_simulation_result(m: &Vec<Monkey>) -> usize {
    m.iter().map(|m| m.test).sorted_unstable_by(|a, b| b.cmp(a)).take(2).product()
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    for num_rounds in vec![20, 10_000] {
        let mut m = parse_input(input);
        let (mo, mut bags) = setup_simulation(&m);
        simulate_round(&mut m, &mut bags, mo, 0..num_rounds, num_rounds == 20);
        println!("The level of monkey business after {} rounds: {}", num_rounds, get_simulation_result(&m));
    }
}
