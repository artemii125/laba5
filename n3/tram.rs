use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

struct TramManager {
    tram_to_stops: HashMap<String, Vec<String>>,
    stop_to_trams: HashMap<String, HashSet<String>>,
}

impl TramManager {
    fn new() -> Self {
        Self {
            tram_to_stops: HashMap::new(),
            stop_to_trams: HashMap::new(),
        }
    }

    fn create_tram(&mut self, tram: String, stops: Vec<String>) {
        if self.tram_to_stops.len() >= 1000 {
            println!("Ошибка: достигнуто максимальное количество трамваев (1000).{}");
            return;
        }
        let mut unique_stops = Vec::new();
        for stop in stops {
            if !unique_stops.contains(&stop) {
                unique_stops.push(stop);
            }
        }
        if unique_stops.len() < 2 {
            println!("Ошибка: должно быть минимум 2 уникальные остановки.");
            return;
        }
        self.tram_to_stops.insert(tram.clone(), unique_stops.clone());
        for stop in unique_stops {
            self.stop_to_trams.entry(stop).or_default().insert(tram.clone());
        }
    }

    fn print_trams_in_stop(&self, stop: &str) {
        if self.tram_to_stops.is_empty() {
            println!("Trams is absent");
            return;
        }
        match self.stop_to_trams.get(stop) {
            Some(trams) => {
                for tram in trams {
                    print!("{} ", tram);
                }
                println!();
            }
            None => println!("Stop is absent"),
        }
    }

    fn print_stops_in_tram(&self, tram: &str) {
        if self.tram_to_stops.is_empty() {
            println!("Trams is absent");
            return;
        }
        match self.tram_to_stops.get(tram) {
            Some(stops) => {
                for stop in stops {
                    print!("Stop {}: ", stop);
                    let mut found = false;
                    if let Some(trams) = self.stop_to_trams.get(stop) {
                        for other in trams {
                            if other != tram {
                                print!("{} ", other);
                                found = true;
                            }
                        }
                    }
                    if !found {
                        print!("0");
                    }
                    println!();
                }
            }
            None => println!("No tram"),
        }
    }

    fn print_all_trams(&self) {
        if self.tram_to_stops.is_empty() {
            println!("Trams is absent");
            return;
        }
        for (tram, stops) in &self.tram_to_stops {
            print!("TRAM {}: ", tram);
            for stop in stops {
                print!("{} ", stop);
            }
            println!();
        }
    }
}

fn main() {
    let mut manager = TramManager::new();
    println!("Введите команды: CREATE_TRAM, TRAMS_IN_STOP, STOPS_IN_TRAM, TRAMS");

    let stdin = io::stdin();
    let mut buffer = String::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        let mut parts = buffer.trim().split_whitespace();
        if let Some(command) = parts.next() {
            match command {
                "CREATE_TRAM" => {
                    if let (Some(tram), Some(count_str)) = (parts.next(), parts.next()) {
                        if let Ok(count) = count_str.parse::<usize>() {
                            let stops: Vec<String> = parts.take(count).map(|s| s.to_string()).collect();
                            if stops.len() == count {
                                manager.create_tram(tram.to_string(), stops);
                            } else {
                                println!("Ошибка: указано {} остановок, но введено меньше.", count);
                            }
                        }
                    }
                }
                "TRAMS_IN_STOP" => {
                    if let Some(stop) = parts.next() {
                        manager.print_trams_in_stop(stop);
                    }
                }
                "STOPS_IN_TRAM" => {
                    if let Some(tram) = parts.next() {
                        manager.print_stops_in_tram(tram);
                    }
                }
                "TRAMS" => manager.print_all_trams(),
                _ => println!("Неизвестная команда: {}", command),
            }
        }
        buffer.clear();
    }
}