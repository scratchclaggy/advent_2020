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

    let modulo = bus_routes.iter().fold(1, |acc, br| acc * br.route_num);
    let global_offset = bus_routes[bus_routes.len() - 1].offset;
    bus_routes.iter_mut().for_each(|br| {
        br.set_moduli(modulo);
        br.offset = global_offset - br.offset;
    });
    let ans =
        bus_routes.iter().fold(0, |acc, br| acc + br.get_product()) % modulo - bus_routes[0].offset;

    println!("Answer: {}", ans);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
struct BusRoute {
    route_num: u64,
    offset: u64,
    moduli: u64,
    inverse: u64,
}

impl BusRoute {
    fn new(route_num: u64, offset: u64) -> BusRoute {
        BusRoute {
            route_num,
            offset,
            moduli: 1,
            inverse: 1,
        }
    }

    fn set_moduli(&mut self, modulo: u64) {
        self.moduli = modulo / self.route_num;
        self.set_inverse();
    }

    fn set_inverse(&mut self) {
        let inverse = self.moduli % self.route_num;
        let mut i = 1;
        while i * inverse % self.route_num != 1 {
            i += 1;
        }

        self.inverse = i;
    }

    fn get_product(&self) -> u64 {
        self.offset * self.moduli * self.inverse
    }
}
