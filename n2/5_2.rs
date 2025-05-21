use rand::Rng;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io::{self, Write};

fn ticket() -> String {
    let num = rand::thread_rng().gen_range(0..1000);
    format!("T{:03}", num)
}

fn main() {
    println!("Доступные команды: ENQUEUE, DISTRIBUTE");
    print!("Введите количество окон: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut tasks: Vec<(String, i32)> = Vec::new();

    loop {
        print!("Команда: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        if command == "ENQUEUE" {
            print!("Введите время: ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let time: i32 = input.trim().parse().unwrap();
            let tick = ticket();
            tasks.push((tick.clone(), time));
            println!("{}", tick);
        } else if command == "DISTRIBUTE" {
            tasks.sort_by(|a, b| b.1.cmp(&a.1));

            let mut window_queues: Vec<Vec<String>> = vec![vec![]; n];
            let mut window_times: Vec<i32> = vec![0; n];

            for (tick, time) in &tasks {
                let min_index = window_times
                    .iter()
                    .enumerate()
                    .min_by_key(|&(_, &v)| v)
                    .map(|(i, _)| i)
                    .unwrap();

                window_queues[min_index].push(tick.clone());
                window_times[min_index] += time;
            }

            for i in 0..n {
                print!("Окно {} ({} минут): ", i + 1, window_times[i]);
                for (j, t) in window_queues[i].iter().enumerate() {
                    print!("{}", t);
                    if j < window_queues[i].len() - 1 {
                        print!(", ");
                    }
                }
                println!();
            }
            break;
        } else {
            println!("Неизвестная команда");
        }
    }
}