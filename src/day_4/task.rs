use std::ops::Range;

fn create_range_from_string(range: &str) -> Range<u32> {
    let range: Vec<&str> = range.split("-").collect();
    let start = range[0].parse::<u32>().unwrap();
    let end = range[1].parse::<u32>().unwrap();

    return start..end;
}

// Part 1
fn check_if_range_fully_contains_numbers(range1: &Range<u32>, range2: &Range<u32>) -> bool {
    if range1.start <= range2.start && range1.end >= range2.end || range2.start <= range1.start && range2.end >= range1.end {
        return true;
    }

    return false;
}

// Part 2
fn check_if_range_overlap_at_all(range1: &Range<u32>, range2: &Range<u32>) -> bool {
    if range1.start <= range2.start && range1.end >= range2.start || range2.start <= range1.start && range2.end >= range1.start {
        return true;
    }

    return false;
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Summe der Zahlenpaare die sich überschneiden (Part 1 & Part2) als Tuple
    let mut sum_of_ranges = (0, 0);

    // Gehe jede Zeile durch
    for line in input.lines() {
        // Teile die Zeile in zwei Teile, das Komma wird dabei entfernt
        let pair = line.split(',').collect::<Vec<&str>>();

        // Erstelle Range Objekte aus den Strings
        let range1 = create_range_from_string(pair[0]);
        let range2 = create_range_from_string(pair[1]);

        // Vergleiche ob sich eine der beiden Ranges vollständig überschneidet
        if check_if_range_fully_contains_numbers(&range1, &range2) {
            sum_of_ranges.0 += 1;
        }

        // Vergleiche ob sich eine der beiden Ranges überhaupt überschneidet
        if check_if_range_overlap_at_all(&range1, &range2) {
            sum_of_ranges.1 += 1;
        }
    }

    // Gib die Summen aus (Part 1 & Part 2)
    println!("Count of assignments which fully contains the other: {}", sum_of_ranges.0);
    println!("Count of assignments which overlap at all: {}", sum_of_ranges.1);
}
