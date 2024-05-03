# Tile Matching Game

This is a simple command-line tile matching game written in Rust. The objective of the game is to match pairs of tiles by guessing their positions on the board.

## How to Play

1. **Generating the Board**: 
   - At the start of the game, a board is generated with pairs of shuffled tiles. Each tile has an associated image.

2. **Game Loop**:
   - The game proceeds in a loop until all tiles are matched.
   - In each iteration of the loop:
     - The screen is cleared.
     - The current state of the board is rendered, displaying visible tiles and hidden tiles represented by dots.
     - The player inputs the indices of two tiles they want to reveal.
     - The selected tiles are momentarily revealed.
     - If the images on the selected tiles match, a message indicating a match is printed. Otherwise, the tiles are covered again after a brief pause.
     - The game checks if all tiles are matched. If so, a congratulatory message is printed, and the game ends.

3. **Input Handling**:
   - The player is prompted to enter the indices of the tiles they want to reveal.
   - If the player enters an invalid index or selects a tile that is already matched, appropriate error messages are displayed.

4. **Winning the Game**:
   - The game ends when all pairs of tiles are matched.
   - Upon winning, the player is congratulated, and the game loop exits.

## Game Components

- **Tile Struct**: Represents a single tile on the game board. Each tile has an image, a visibility status (whether it's visible or hidden), and an image path.

## Code Structure

The code is organized into functions and a `Tile` struct:
- **Main Function**: Contains the game loop and overall game logic.
- **Tile Struct**: Defines the properties and methods for each tile.
- **Helper Functions**: Functions for generating the board, rendering the board, handling player input, and revealing/covering tiles.

## Dependencies

- `rand`: Used for shuffling the tiles on the board.
- `std::io`: Used for input/output operations.
- `std::thread::sleep` and `std::time::Duration`: Used for creating brief pauses in the game flow.

## How to Run

1. Clone the repository.
2. Navigate to the directory containing the source code.
3. Run `cargo build` to build the executable.
4. Run `cargo run` to start the game.

## Notes

- The game board is fixed at a size of 3x3.
- The number of tile pairs is fixed at 6, resulting in a total of 12 tiles on the board.
- The game logic does not include any complex scoring or timing mechanisms. It focuses on the core gameplay of matching tiles.
