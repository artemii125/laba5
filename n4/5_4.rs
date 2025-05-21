use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let mut regions: HashMap<String, String> = HashMap::new();
    let mut deleted_names: HashSet<String> = HashSet::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    input.clear();

    for _ in 0..n {
        print!("");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();

        match command {
            "CHANGE" => {
                let region = parts.next().unwrap().to_string();
                let new_center = parts.next().unwrap().to_string();

                if let Some(old_center) = regions.get(&region) {
                    if *old_center == new_center {
                        println!("Incorrect");
                    } else {
                        regions.insert(region.clone(), new_center.clone());
                        println!(
                            "Region {} has changed its administrative center from {} to {}",
                            region, old_center, new_center
                        );
                    }
                } else if deleted_names.contains(&region) {
                    println!("Incorrect");
                } else {
                    regions.insert(region.clone(), new_center.clone());
                    println!(
                        "New region {} with administrative center {}",
                        region, new_center
                    );
                }
            }

            "RENAME" => {
                let old_region = parts.next().unwrap().to_string();
                let new_region = parts.next().unwrap().to_string();

                if old_region == new_region
                    || !regions.contains_key(&old_region)
                    || regions.contains_key(&new_region)
                {
                    println!("Incorrect");
                } else {
                    let center = regions.remove(&old_region).unwrap();
                    regions.insert(new_region.clone(), center);
                    deleted_names.insert(old_region.clone());
                    println!("{} has been renamed to {}", old_region, new_region);
                }
            }

            "ABOUT" => {
                let region = parts.next().unwrap().to_string();
                if let Some(center) = regions.get(&region) {
                    println!("{} has administrative center {}", region, center);
                } else {
                    println!("Incorrect");
                }
            }

            "ALL" => {
                for (region, center) in &regions {
                    println!("{} - {}", region, center);
                }
            }

            _ => println!("Unknown command"),
        }
    }
}
