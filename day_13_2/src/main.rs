use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let input_file = fs::read_to_string(FILENAME).expect("FILE ERROR");
    let mut bus_routes: Vec<BusRoute> = input_file
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(off, br)| {
            if br != "x" {
                Some(BusRoute::new(br.parse().unwrap(), off as u64))
            } else {
                None
            }
        })
        .collect();

    bus_routes.sort();

    let mut first_route = bus_routes[bus_routes.len() - 1];

    // for route in bus_routes.iter() {
    //     println!("{:03}: {}", route.route_num, route.arrival_time());
    // }
    'restart: loop {
        for route in bus_routes.iter_mut().rev().skip(1) {
            if first_route.arrival_time() != route.arrival_time() {
                while first_route.arrival_time() != route.arrival_time() {
                    // println!(
                    //     "{:64} : {:64}",
                    //     first_route.arrival_time(),
                    //     route.arrival_time()
                    // );
                    if first_route.arrival_time() < route.arrival_time() {
                        first_route.mult += 1;
                    } else {
                        route.mult = first_route.arrival_time() / route.route_num + route.route_num;
                    }
                }
                continue 'restart;
            }
        }

        break;
    }

    // for route in bus_routes.iter() {
    //     println!("{:03}: {}", route.route_num, route.arrival_time());
    // }

    println!("Answer: {}", first_route.arrival_time());
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
struct BusRoute {
    route_num: u64,
    mult: u64,
    offset: u64,
}

impl BusRoute {
    fn new(route_num: u64, offset: u64) -> BusRoute {
        BusRoute {
            route_num,
            mult: 5_000_000_000 / route_num,
            offset,
        }
    }

    fn arrival_time(&self) -> u64 {
        self.route_num * self.mult - self.offset
    }
}
