use nom::*;

const MAP_DIMENSIONS: (usize, usize) = (700, 200);

// Diese Funktion wird verwendet, um einen usize-Wert (eine Ganzzahl) aus einem Eingabestring zu erhalten
// Die Analyse findet durch den Nom-Parser statt
fn usize(input: &[u8]) -> Result<(&[u8], usize), nom::Err<(&[u8], nom::error::ErrorKind)>> {
    map_opt!(input, nom::character::complete::digit1, atoi::atoi)
}

// Diese Funktion wird verwendet, um eine Koordinate (x,y) aus dem Eingabestring zu erhalten
// Die Analyse findet durch den Nom-Parser statt
fn coord(input: &[u8]) -> Result<(&[u8], (usize, usize)), nom::Err<(&[u8], nom::error::ErrorKind)>> {
    separated_pair!(input, usize, char!(','), usize)
}

// Diese Funktion wird verwendet, um eine Liste von Koordinaten aus dem Eingabestring zu erhalten.
// Die Analyse findet durch den Nom-Parser statt.
fn line(input: &[u8]) -> Result<(&[u8], Vec<(usize, usize)>), nom::Err<(&[u8], nom::error::ErrorKind)>> {
    separated_list1!(input, nom::bytes::complete::tag(" -> "), coord).map(|(remaining_input, coords)| {
        let coords: Vec<_> = coords.into_iter().map(|(x, y)| (x, y)).collect();
        (remaining_input, coords)
    })
}

// Diese Funktion wird verwendet, um die nächste nicht gefüllte Koordinate in einer Map zu tracken
// Die Map ist ein Array aus bool-Werten
#[inline]
fn track(map: &[[bool; MAP_DIMENSIONS.0]; MAP_DIMENSIONS.1], (mut x, y): (usize, usize), low: Option<usize>) -> Option<(usize, usize)> {
    (y..MAP_DIMENSIONS.1 - 1)
        .find(|y| {
            low.map_or(false, |low| *y == low - 1)
                || [x, x - 1, x + 1]
                .into_iter()
                .find(|x| !map[*y + 1][*x])
                .map(|xx| x = xx)
                .is_none()
        })
        .map(|y| (x, y))
}


// Diese Funktion wird verwendet, um die Ergebnisse aus dem Eingabestring auszugeben
// Es wird eine Map (Array aus bool-Werten) und ein Wert für den Sand erstellt, der sich abgesetzt hat
#[inline]
fn print_results(input: &[u8], mut map: [[bool; MAP_DIMENSIONS.0]; MAP_DIMENSIONS.1], mut sand: usize) {
    let lines = input
        .split(|b| b == &b'\n')
        .filter(|b| !b.is_empty())
        .map(|l| line(l).unwrap().1)
        .collect::<Vec<_>>();

    for coords in &lines {
        coords.array_windows().for_each(|[a, b]| {
            if a.0 == b.0 {
                (a.1.min(b.1)..=a.1.max(b.1)).for_each(|y| map[y][a.0] = true);
            } else {
                (a.0.min(b.0)..=a.0.max(b.0)).for_each(|x| map[a.1][x] = true);
            }
        });
    }

    while let Some((x, y)) = track(&map, (500, 0), None) {
        map[y][x] = true;
        sand += 1;
    }

    println!("Units of sand come to rest before sand starts flowing into the abyss: {}", sand);

    let mut lowest = 0;
    for coords in &lines {
        coords.array_windows().for_each(|[a, b]| {
            lowest = lowest.max(a.1.max(b.1));
            if a.0 == b.0 {
                (a.1.min(b.1)..=a.1.max(b.1)).for_each(|y| map[y][a.0] = true);
            } else {
                (a.0.min(b.0)..=a.0.max(b.0)).for_each(|x| map[a.1][x] = true);
            }
        });
    }

    while let Some((x, y)) = track(&map, (500, 0), Some(lowest + 2)) {
        if y == 0 {
            break;
        }
        sand += 1;
        map[y][x] = true;
    }

    println!("Units of falling sand until the source of the sand becomes blocked: {}", sand + 1);
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_bytes!("input.txt");

    // Gebe die Ergebnisse aus
    print_results(input, [[false; MAP_DIMENSIONS.0]; MAP_DIMENSIONS.1], 0);
}
