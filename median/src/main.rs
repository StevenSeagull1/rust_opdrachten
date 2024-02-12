use std::collections::HashMap;
fn main() {
    let mut numbers = vec![
        1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9,
        9, 9,
    ];

    let median = median(&mut numbers);
    println!("median: {}", median);

    let mode = mode(&mut numbers);
    println!("mode: {}", mode);
}

fn median(numbers: &mut Vec<i32>) -> f64 {
    numbers.sort();
    let len = numbers.len();
    //als de lijst een even lengte heeft berekent het de gemmidelde van de 2 middelste getallen
    if len % 2 == 0 {
        let middle_links = numbers[len / 2 - 1] as f64;
        let middle_rechts = numbers[len / 2] as f64;
        (middle_links + middle_rechts) / 2.0
    } else {
        // als de lijst oneven lang is dan pakt hij het middelste getal
        numbers[len / 2] as f64
    }
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut frequency_map: HashMap<i32, usize> = HashMap::new();
    // zet alle nummers in een hash map
    for &num in numbers {
        *frequency_map.entry(num).or_insert(0) += 1;
    }
    //telt hoevaak dezelfde nummer erin zit
    let mut mode = 0;
    let mut max_frequency = 0;
    for (&num, &frequency) in &frequency_map {
        if frequency > max_frequency {
            mode = num;
            max_frequency = frequency;
        }
    }
    mode
}
