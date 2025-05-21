use std::collections::HashMap;
use std::io::{self, Write};

const MAX_PRODUCTS: i32 = 10;
const ZONES: i32 = 10;
const RACK: i32 = 3;
const SECTIONS: i32 = 1;
const SHELVES: i32 = 5;
const CELLS: i32 = ZONES * RACK * SECTIONS * SHELVES;

type Storage = HashMap<String, HashMap<String, i32>>;

fn check(address: &str) -> bool {
    if address.len() != 4 {
        return false;
    }
    let chars: Vec<char> = address.chars().collect();
    (chars[0] >= 'A' && chars[0] <= 'J') &&
    (chars[1] >= '1' && chars[1] <= '3') &&
    chars[2] == '1' &&
    (chars[3] >= '1' && chars[3] <= '5')
}

fn add(storage: &mut Storage, name: &str, qty: i32, address: &str) {
    if !check(address) {
        println!("Неправильный адрес\n");
        return;
    }

    let mut places = 0;
    if let Some(cell) = storage.get(address) {
        for value in cell.values() {
            places += value;
        }
    }

    if places + qty > MAX_PRODUCTS {
        println!("Ошибка: ячейка {} вмещает только {} шт.", address, MAX_PRODUCTS);
        println!("Сейчас: {}\n", places);
        return;
    }

    storage
        .entry(address.to_string())
        .or_default()
        .entry(name.to_string())
        .and_modify(|e| *e += qty)
        .or_insert(qty);

    println!("Добавлено {} в количестве {} шт. в ячейку {}\n", name, qty, address);
}

fn remove(storage: &mut Storage, name: &str, qty: i32, address: &str) {
    if !check(address) {
        println!("Неправильный адрес\n");
        return;
    }

    if !storage.contains_key(address) || !storage[address].contains_key(name) {
        println!("Ошибка: товара {} нет в ячейке {}\n", name, address);
        return;
    }

    if storage[address][name] < qty {
        println!("Ошибка: недостаточно {} в ячейке {}\n", name, address);
        return;
    }

    if let Some(cell) = storage.get_mut(address) {
        if let Some(entry) = cell.get_mut(name) {
            *entry -= qty;
            if *entry == 0 {
                cell.remove(name);
            }
        }
        if cell.is_empty() {
            storage.remove(address);
        }
    }

    println!("Удалено {} в количестве {} шт. из ячейки {}\n", name, qty, address);
}

fn info(storage: &Storage) {
    let used: i32 = storage
        .values()
        .flat_map(|cell| cell.values())
        .sum();

    let percent = (used as f64) * 100.0 / (CELLS as f64);
    println!("Заполненность склада: {:.2}%", percent);
    println!("Содержимое занятых ячеек:");

    for (address, contents) in storage {
        println!("{}:", address);
        for (name, qty) in contents {
            println!("{} = {}", name, qty);
        }
        println!();
    }

    let empty = CELLS - storage.len() as i32;
    println!("Пустых ячеек: {}", empty);
}

fn main() {
    let mut storage: Storage = HashMap::new();
    println!("Перечень команд: ADD, REMOVE, INFO\n");

    loop {
        print!("Введите команду: ");
        io::stdout().flush().unwrap();

        let mut command_line = String::new();
        io::stdin().read_line(&mut command_line).unwrap();
        let parts: Vec<&str> = command_line.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["ADD", name, qty_str, address] => {
                if let Ok(qty) = qty_str.parse::<i32>() {
                    add(&mut storage, name, qty, address);
                } else {
                    println!("Некорректное количество\n");
                }
            }
            ["REMOVE", name, qty_str, address] => {
                if let Ok(qty) = qty_str.parse::<i32>() {
                    remove(&mut storage, name, qty, address);
                } else {
                    println!("Некорректное количество\n");
                }
            }
            ["INFO"] => {
                info(&storage);
            }
            _ => {
                println!("Неизвестная команда\n");
            }
        }
    }
}
