use battleship::board::{Cell, SIZE};
use battleship::data::{ScoreRecord, ShipSpec, parse_board, parse_score, parse_ship_line};

// A valid board as text. Row 0 starts with two ship cells; row 1 has one at column 2.
const GOOD_BOARD: &str = "SS........\n\
                          ..S.......\n\
                          ..........\n\
                          ..........\n\
                          ..........\n\
                          ..........\n\
                          ..........\n\
                          ..........\n\
                          ..........\n\
                          .........S\n";

// ---- PROVIDED tests (they fail until you write the matching function) ----

#[test]
fn parse_ship_line_valid() {
    assert_eq!(
        parse_ship_line("Carrier,5"),
        Some(ShipSpec {
            name: "Carrier".to_string(),
            length: 5
        })
    );
}

#[test]
fn parse_ship_line_trims_spaces() {
    assert_eq!(
        parse_ship_line("  Cruiser , 3 "),
        Some(ShipSpec {
            name: "Cruiser".to_string(),
            length: 3
        })
    );
}

#[test]
fn parse_ship_line_rejects_missing_comma() {
    assert_eq!(parse_ship_line("Flagship"), None);
}

#[test]
fn parse_ship_line_rejects_length_one() {
    assert_eq!(parse_ship_line("Tugboat,1"), None);
}

#[test]
fn parse_board_valid() {
    let board = parse_board(GOOD_BOARD).expect("GOOD_BOARD should parse");
    assert_eq!(board[0][0], Cell::Ship);
    assert_eq!(board[0][1], Cell::Ship);
    assert_eq!(board[0][2], Cell::Water);
    assert_eq!(board[1][2], Cell::Ship);
    assert_eq!(board[SIZE - 1][SIZE - 1], Cell::Ship);
}

#[test]
fn parse_score_valid() {
    assert_eq!(
        parse_score("Ana,83"),
        Some(ScoreRecord {
            name: "Ana".to_string(),
            score: 83
        })
    );
}

// ---- STUDENT tests: write these three (Task 11) ----
// 1. parse_ship_line returns None for a length of 6 ("Titan,6").
// 2. parse_ship_line returns None when the length is not a number ("Dinghy,x").
// 3. parse_board returns None when there are only nine rows. Write the nine
//    rows as a string literal like GOOD_BOARD, with one row removed.
