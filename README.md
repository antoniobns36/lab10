# Battleship

A solo Battleship game: find and sink a hidden fleet on a 10×10 board, using as few shots as you can. The fleet, a practice board, and the high-score list all live in files. You will extend this one project across the remaining weekly labs. Each week starts from the instructor's solution to the previous week.

## Run

```
cargo run
cargo test
cargo build      # must finish with zero warnings
```

Run from the project folder, not from `src/`: the program opens files in `data/` by a relative path.

## Layout

| File | Edit this lab? |
|---|---|
| `src/board.rs` | No. Board rules, the 2D array, firing, rendering. Provided. |
| `src/ui.rs` | No. Prompts and menus. Provided. |
| `src/data.rs` | Yes. Six functions to write. |
| `src/main.rs` | Yes. Wire the data functions into the menu. |
| `tests/board_tests.rs` | No. Passes now. |
| `tests/data_tests.rs` | Yes. Add three tests. |
| `data/fleet.txt` | Read it. Do not fix the bad lines by hand. |
| `data/boards/practice.txt` | Read it. A fixed 10×10 layout. |

The board is `[[Cell; 10]; 10]`, indexed `board[row][col]`. Row 0 is the top row, shown as 1. Column 0 is A. So `B7` means `board[6][1]`.

## Lab 10 tasks (work in this order, commit after each)

1. `parse_ship_line` (flowchart first). Four `parse_ship_line` tests should now pass.
2. `load_fleet`.
3. `parse_board` (flowchart first). The `parse_board_valid` test should pass.
4. `load_board`.
5. In `main.rs`, switch to `data::load_fleet(FLEET_PATH)`. Delete `default_fleet`. Run the game; you should see `Skipped 3 invalid line(s)`.
6. `append_score`.
7. `load_scores`.
8. Practice mode in `main.rs`.
9. Top-5 scores on the menu screen.
10. Save the score after a win.

## Rules

- `cargo build` shows no warnings.
- At least one commit per function you write.
- Do not change `board.rs`, `ui.rs`, or the provided tests.
