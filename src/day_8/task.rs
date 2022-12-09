// Funktion um die sichtbaren Bäume zu ermitteln
fn get_visible_trees(grid: &Vec<Vec<char>>) -> usize {
    // Anzahl der sichtbaren Bäume
    let mut visible_trees = 0;

    // Gehe das Grid durch
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {

            // Prüfen, ob der Baum von links, rechts, oben oder unten sichtbar ist
            if grid.iter().take(y).all(|row| row[x] < grid[y][x]) || grid.iter().skip(y + 1).all(|row| row[x] < grid[y][x])
                || grid[y].iter().take(x).all(|&tree| tree < grid[y][x]) || grid[y].iter().skip(x + 1).all(|&tree| tree < grid[y][x]) {
                // Wenn ja, erhöhe die Anzahl der sichtbaren Bäume
                visible_trees += 1;
            }
        }
    }

    return visible_trees;
}

// Funktion um die Punkte zu ermitteln
fn get_scenic_score(grid: &Vec<Vec<char>>) -> usize {
    // Anzahl der Punkte
    let mut scenic_score = 0;

    // Gehe das Grid durch
    for y in 1..grid.len() - 1 {
        for x in 1..grid.len() - 1 {
            let value = &grid[y][x];
            let row = &grid[y];
            let col = grid.iter().map(|row| row[x]).collect::<Vec<char>>();

            let left_num = row[0..x].iter().rev().position(|el| el >= value).unwrap_or(x - 1) + 1;
            let right_num = row[x + 1..row.len()].iter().position(|el| el >= value).unwrap_or(row.len() - 1 - (x + 1)) + 1;
            let up_num = col[0..y].iter().rev().position(|el| el >= value).unwrap_or(y - 1) + 1;
            let down_num = col[y + 1..col.len()].iter().position(|el| el >= value).unwrap_or(col.len() - 1 - (y + 1)) + 1;

            let metric = left_num * right_num * up_num * down_num;
            if metric > scenic_score {
                scenic_score = metric;
            }
        }
    }

    return scenic_score;
}

pub fn main() {
    // Lese die input.txt Datei ein
    let input = include_str!("input.txt");

    // Erstelle multi-dimensionalen Vector
    let mut grid: Vec<Vec<char>> = Vec::new();

    // Gehe jede Zeile durch
    for line in input.lines() {
        // Füge die Zeile dem Grid hinzu
        grid.push(line.chars().collect());
    }

    // Gebe die Anzahl der sichtbaren Bäume aus
    println!("Number of visible trees: {}", get_visible_trees(&grid));
    // Gebe die höchsten Punkte aus
    println!("Highest score: {}", get_scenic_score(&grid));
}
