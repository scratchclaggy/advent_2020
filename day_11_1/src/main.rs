fn main() {
    let occupied_seat = Seat::Occupied;
    let another_occupied_seat = Seat::Occupied;
    let vacant_seat = Seat::Vacant;

    let vec1 = vec![occupied_seat];
    let vec2 = vec![another_occupied_seat];

    println!("Occupied = occupied: {}", vec1 == vec2);
}

#[derive(PartialEq)]
enum Seat {
    Vacant,
    Occupied,
    Floor,
}
