use battleship::board::{self, Board, Shot};
use battleship::data::{self, ShipSpec};
use battleship::ui;
use rand::SeedableRng;
use rand::rngs::StdRng;

const FLEET_PATH: &str = "data/fleet.txt";
// TODO (Task 6): add SCORES_PATH ("data/scores.txt").
// TODO (Task 8): add PRACTICE_PATH ("data/boards/practice.txt").
const TOP_N: usize = 5;

fn main() {
    // TODO (Task 5): replace default_fleet() with data::load_fleet(FLEET_PATH),
    // then delete default_fleet from data.rs.
    let fleet = data::default_fleet();
    if fleet.is_empty() {
        println!("No fleet loaded. Check {FLEET_PATH}.");
        return;
    }

    let menu = vec![
        String::from("Play"),
        String::from("Practice board"),
        String::from("Quit"),
    ];
    loop {
        println!("\n=== BATTLESHIP ===");
        show_top_scores(TOP_N);
        match ui::menu_choice("\nChoose:", &menu) {
            1 => play_game(&fleet),
            2 => practice_game(),
            _ => {
                println!("Goodbye.");
                break;
            }
        }
    }
}

// Print the best scores, highest first, or a message if there are none.
fn show_top_scores(n: usize) {
    // TODO (Task 9): load the scores; print "TOP n"; if there are none print
    // "  no scores yet"; otherwise sort with data::sort_scores_desc and print up
    // to n lines like
    //  1. Ana              83
    println!("TOP {n}");
    println!("  (high scores are not shown yet)");
}

// A scored game against a randomly placed fleet.
fn play_game(fleet: &[ShipSpec]) {
    let mut board = board::new_board();
    let mut rng = StdRng::seed_from_u64(rand::random::<u64>());
    if !board::place_fleet(&mut board, fleet, &mut rng) {
        println!("The fleet does not fit on the board. Check {FLEET_PATH}.");
        return;
    }
    if let Some(shots) = play_round(&mut board) {
        let score = board::score_for(shots);
        println!("All ships sunk in {shots} shots. Score: {score}");
        // TODO (Task 10): if score > 0, ask for a name with ui::prompt, build a
        // ScoreRecord (add it to the `use` line), and save it with
        // data::append_score. Print "Score saved." only when it returns true.
    }
}

// An unscored game on the fixed layout in the practice file.
fn practice_game() {
    // TODO (Task 8): load the board with data::load_board(PRACTICE_PATH). On
    // Some(mut board), play it with play_round and print
    // "Practice board cleared in N shots." when play_round returns Some(N).
    // On None, print "Practice board unavailable."
    println!("Practice mode is not available yet.");
}

// Fire until every ship is sunk. Returns the number of shots, or None if the
// player types "quit". PROVIDED.
fn play_round(board: &mut Board) -> Option<u32> {
    let mut shots = 0;
    while !board::all_sunk(board) {
        println!();
        print!("{}", board::render(board, false));
        let text = ui::prompt("Fire at (like B7, or 'quit'): ");
        if text.eq_ignore_ascii_case("quit") {
            return None;
        }
        match board::parse_coord(&text) {
            None => println!("Enter a column A-J and a row 1-10, like B7."),
            Some((row, col)) => match board::fire(board, row, col) {
                Shot::Hit => {
                    shots += 1;
                    println!("Hit!");
                }
                Shot::Miss => {
                    shots += 1;
                    println!("Miss.");
                }
                Shot::AlreadyShot => println!("You already fired there."),
            },
        }
    }
    print!("\n{}", board::render(board, false));
    Some(shots)
}
