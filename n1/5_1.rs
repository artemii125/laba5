use std::collections::VecDeque;
use std::io::{self, Write};
use rand::Rng;

fn ticket() -> String {
    let num = rand::thread_rng().gen_range(0..1000);
    format!("T{:03}", num)
}

fn main() {
    let mut tickets: VecDeque<String> = VecDeque::new();
    let mut durations: VecDeque<i32> = VecDeque::new();

    println!("Доступные команды: ENQUEUE, DISTRIBUTE");
    print!("Введите количество окон: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

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
            durations.push_back(time);
            tickets.push_back(tick.clone());
            println!("{}", tick);
        } else if command == "DISTRIBUTE" {
            let mut window_time = vec![0; n];
            let mut window_queue: Vec<Vec<String>> = vec![vec![]; n];

            while let (Some(t), Some(d)) = (tickets.pop_front(), durations.pop_front()) {
                let min_index = window_time
                    .iter()
                    .enumerate()
                    .min_by_key(|&(_, &time)| time)
                    .map(|(i, _)| i)
                    .unwrap();

                window_time[min_index] += d;
                window_queue[min_index].push(t);
            }

            for (i, (time, queue)) in window_time.iter().zip(window_queue.iter()).enumerate() {
                print!("Окно {} ({} минут): ", i + 1, time);
                for (j, t) in queue.iter().enumerate() {
                    print!("{}", t);
                    if j < queue.len() - 1 {
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
