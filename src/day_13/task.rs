use nom::*;
use std::cmp::Ordering;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq)]
enum Item {
    I(u8),
    L(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::I(a), Item::I(b)) => a.cmp(b),
            (Item::L(a), Item::L(b)) => match a.iter().cmp(b) {
                r if r != Ordering::Equal => r,
                _ => a.len().cmp(&b.len()),
            },
            (Item::I(_), Item::L(b)) if b.len() == 1 => self.cmp(&b[0]),
            (Item::I(a), Item::L(_)) => Item::L(vec![Item::I(*a)]).cmp(other),
            (Item::L(_), Item::I(_)) => other.cmp(self).reverse(),
        }
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn item(input: &[u8]) -> Result<(&[u8], Item), nom::Err<(&[u8], nom::error::ErrorKind)>> {
    // Die alt!-Funktion wird verwendet, um zwischen zwei Alternativen zu wählen
    // In diesem Fall wird entweder ein List-Element oder eine Nummer zurückgegeben
    alt!(input, map!(list, Item::L) | map!(num, Item::I))
}

fn num(input: &[u8]) -> Result<(&[u8], u8), nom::Err<(&[u8], nom::error::ErrorKind)>> {
    // Die map_opt!-Funktion wird verwendet, um ein Option-Element als Eingabe zu verwenden
    // In diesem Fall wird die Funktion atoi::atoi aufgerufen, um einen String in eine Ganzzahl zu konvertieren
    map_opt!(input, nom::character::complete::digit1, atoi::atoi)
}

// Diese Funktion liest eine Liste von Elementen ein
fn list(input: &[u8]) -> Result<(&[u8], Vec<Item>), nom::Err<(&[u8], nom::error::ErrorKind)>> {
    // Die delimited!-Funktion wird verwendet, um einen Bereich in einem Eingabestring zu markieren
    // In diesem Fall wird ein Bereich von eckigen Klammern markiert
    // Anschließend werden die Elemente in der Liste über die separated_list0!-Funktion eingelesen
    delimited!(input, char!('['), separated_list0!(char!(','), item), char!(']'))
}

// Diese Funktion liest ein Paar von Elementen ein
fn pair(input: &[u8]) -> Result<(&[u8], (Item, Item)), nom::Err<(&[u8], nom::error::ErrorKind)>> {
    // Die separated_pair!-Funktion wird verwendet, um zwei Elemente zu trennen, die durch ein Trennzeichen getrennt sind
    // In diesem Fall wird ein Zeilenumbruch als Trennzeichen verwendet
    separated_pair!(input, item, tag!("\n"), item)
}

lazy_static! {
    static ref FIRST: Item = Item::L(vec![Item::L(vec![Item::I(2)])]);
    static ref SECOND: Item = Item::L(vec![Item::L(vec![Item::I(6)])]);
}

// Diese Funktion gibt die Ergebnisse aus
fn print_results(input: &str) {
    // Die Daten werden in ein Vector umgewandelt
    // Anschließend werden die Elemente, die kleiner als SECOND sind, aus dem Vector entfernt
    let packets: Vec<Item> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| item(l.as_bytes()).unwrap().1)
        .filter(|i| i < &SECOND)
        .collect();

    println!(
        "Sum of the indices of those pairs: {}", input
            .split("\n\n")
            .map(|p| pair(p.as_bytes()).unwrap().1)
            .enumerate()
            .filter(|(_, (a, b))| a.cmp(b) == Ordering::Less)
            .map(|(i, _)| i + 1)
            .sum::<usize>(),
    );

    println!("Decoder key for the distress signal: {}",
             (packets.iter().filter(|i| *i < &FIRST).count() + 1) * (packets.len() + 2)
    );
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Gebe die Ergebnisse aus
    print_results(input);
}
