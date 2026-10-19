// PROVIDED: tests for the board rules. They pass in the starter.
use battleship::board::{
    Board, Cell, SIZE, Shot, all_sunk, can_place, count_cells, fire, new_board, parse_coord, place,
    place_fleet, render, score_for,
};
use battleship::data::ShipSpec;
use rand::SeedableRng;
use rand::rngs::StdRng;

fn spec(name: &str, length: usize) -> ShipSpec {
    ShipSpec {
        name: name.to_string(),
        length,
    }
}

#[test]
fn new_board_is_all_water() {
    let board: Board = new_board();
    assert_eq!(count_cells(&board, Cell::Water), SIZE * SIZE);
}

#[test]
fn can_place_rejects_off_the_edge() {
    let board = new_board();
    assert!(!can_place(&board, 0, 8, 3, true));
    assert!(!can_place(&board, 8, 0, 3, false));
    assert!(can_place(&board, 0, 7, 3, true));
    assert!(can_place(&board, 7, 0, 3, false));
}

#[test]
fn can_place_rejects_overlap() {
    let mut board = new_board();
    place(&mut board, 2, 2, 3, true);
    assert!(!can_place(&board, 0, 3, 4, false));
    assert!(can_place(&board, 3, 2, 3, true));
}

#[test]
fn fire_hit_miss_and_repeat() {
    let mut board = new_board();
    place(&mut board, 0, 0, 2, true);
    assert_eq!(fire(&mut board, 0, 0), Shot::Hit);
    assert_eq!(fire(&mut board, 5, 5), Shot::Miss);
    assert_eq!(fire(&mut board, 0, 0), Shot::AlreadyShot);
    assert_eq!(fire(&mut board, 5, 5), Shot::AlreadyShot);
    assert!(!all_sunk(&board));
    assert_eq!(fire(&mut board, 0, 1), Shot::Hit);
    assert!(all_sunk(&board));
}

#[test]
fn parse_coord_cases() {
    assert_eq!(parse_coord("A1"), Some((0, 0)));
    assert_eq!(parse_coord("b7"), Some((6, 1)));
    assert_eq!(parse_coord(" J10 "), Some((9, 9)));
    assert_eq!(parse_coord("K1"), None);
    assert_eq!(parse_coord("A0"), None);
    assert_eq!(parse_coord("A11"), None);
    assert_eq!(parse_coord("7B"), None);
    assert_eq!(parse_coord("A"), None);
    assert_eq!(parse_coord(""), None);
}

#[test]
fn seeded_placement_is_repeatable_and_complete() {
    let fleet = vec![spec("Carrier", 5), spec("Cruiser", 3), spec("Destroyer", 2)];
    let mut a = new_board();
    let mut b = new_board();
    assert!(place_fleet(&mut a, &fleet, &mut StdRng::seed_from_u64(7)));
    assert!(place_fleet(&mut b, &fleet, &mut StdRng::seed_from_u64(7)));
    assert_eq!(a, b);
    assert_eq!(count_cells(&a, Cell::Ship), 10);
}

#[test]
fn render_hides_ships_unless_revealed() {
    let mut board = new_board();
    place(&mut board, 0, 0, 2, true);
    assert!(!render(&board, false).contains('S'));
    assert!(render(&board, true).contains('S'));
}

#[test]
fn score_for_table() {
    assert_eq!(score_for(17), 83);
    assert_eq!(score_for(50), 50);
    assert_eq!(score_for(100), 0);
    assert_eq!(score_for(150), 0);
}
