pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Vector für int Werte
    let mut numbers: Vec<i32> = Vec::new();
    // Vector für die Summen
    let mut sums: Vec<i32> = Vec::new();

    // Gehe jede Zeile durch
    for line in input.lines() {
        if line.is_empty() {
            // Gebe die gesammelten Werte der Gruppe aus
            println!("Group: {:?}", numbers);
            // Füge die Summe der Gruppe dem Vector sums hinzu
            sums.push(numbers.iter().sum::<i32>());

            // Lösche die gesammelten Werte
            numbers.clear();
        } else {
            // Füge die Zahl der Gruppe hinzu
            numbers.push(line.parse::<i32>().unwrap());
        }
    }

    // Gebe die drei größten Werte von dem Vector sums aus
    sums.sort();
    sums.reverse();
    println!("Highest three sums: {:?}", sums[0..3].to_vec());

    // Summe der drei größten Werte aus dem Vector sums
    println!("Sum of the three highest sums: {:?}", sums[0..3].iter().sum::<i32>());
}
