// Meine Lösungen für den Advent of Code 2022. https://adventofcode.com/2022

#![allow(non_snake_case)]
use chrono::prelude::*;

fn main() {
    // Führe Day Funktion des jeweiligen Tages aus
    // Der Tag wird durch die lokale Zeit bestimmt
    match Local::now().day() {
        1 => day_1::task::main(),
        2 => day_2::task::main(),
        3 => day_3::task::main(),
        4 => day_4::task::main(),
        5 => day_5::task::main(),
        6 => day_6::task::main(),
        // 7 => day_7::task::main(),
        // 8 => day_8::task::main(),
        // 9 => day_9::task::main(),
        // 10 => day_10::task::main(),
        // 11 => day_11::task::main(),
        // 12 => day_12::task::main(),
        // 13 => day_13::task::main(),
        // 14 => day_14::task::main(),
        // 15 => day_15::task::main(),
        // 16 => day_16::task::main(),
        // 17 => day_17::task::main(),
        // 18 => day_18::task::main(),
        // 19 => day_19::task::main(),
        // 20 => day_20::task::main(),
        // 21 => day_21::task::main(),
        // 22 => day_22::task::main(),
        // 23 => day_23::task::main(),
        // 24 => day_24::task::main(),
        // 25 => day_25::task::main(),
        _ => println!("No day found"),
    }
}

mod day_1 {
    pub mod task;
}

mod day_2 {
    pub mod task;
}

mod day_3 {
    pub mod task;
}

mod day_4 {
    pub mod task;
}

mod day_5 {
    pub mod task;
}

mod day_6 {
    pub mod task;
}

// mod day_7 {
//     pub mod task;
// }

// mod day_8 {
//     pub mod task;
// }

// mod day_9 {
//     pub mod task;
// }

// mod day_10 {
//     pub mod task;
// }

// mod day_11 {
//     pub mod task;
// }

// mod day_12 {
//     pub mod task;
// }

// mod day_13 {
//     pub mod task;
// }

// mod day_14 {
//     pub mod task;
// }

// mod day_15 {
//     pub mod task;
// }

// mod day_16 {
//     pub mod task;
// }

// mod day_17 {
//     pub mod task;
// }

// mod day_18 {
//     pub mod task;
// }

// mod day_19 {
//     pub mod task;
// }

// mod day_20 {
//     pub mod task;
// }

// mod day_21 {
//     pub mod task;
// }

// mod day_22 {
//     pub mod task;
// }

// mod day_23 {
//     pub mod task;
// }

// mod day_24 {
//     pub mod task;
// }

// mod day_25 {
//     pub mod task;
// }
