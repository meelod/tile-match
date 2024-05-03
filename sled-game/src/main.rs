use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

const BOARD_SIZE: usize = 3;
const TILE_PAIRS: usize = 6;

fn main() {
    // Generate board with shuffled pairs of tiles
    let mut board = generate_board();

    // Game loop
    loop {
        // Clear the screen before rendering
        clear_screen();

        // Render the board
        render_board(&board);

        // Get player input
        let (first_choice, second_choice) = get_player_input(&board);

        // Reveal selected tiles momentarily
        reveal_tiles(&mut board, first_choice, second_choice);

        // Check if tiles match
        if board[first_choice].image == board[second_choice].image {
            println!("Match found!");
        } else {
            println!("No match, try again.");
            // Cover tiles again after a brief pause
            sleep(Duration::from_secs(1));
            cover_tiles(&mut board, first_choice, second_choice);
        }

        // Check if all tiles are matched
        if board.iter().all(|tile| tile.visible) {
            println!("Congratulations! You matched all pairs.");
            break;
        }

        // Wait for a moment to show the tiles
        sleep(Duration::from_secs(2));
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Tile {
    image: String,
    visible: bool,
    image_path: String,
}

impl Tile {
    fn new(image: String, image_path: String) -> Self {
        Tile {
            image,
            visible: false,
            image_path: image_path.to_string(),
        }
    }
}

fn generate_board() -> Vec<Tile> {
    let mut pairs = Vec::new();
    for i in 0..TILE_PAIRS {
        let image_path = format!("image{}.png", i);
        pairs.push(Tile::new(i.to_string(), image_path.clone()));
        pairs.push(Tile::new(i.to_string(), image_path));
    }

    let mut rng = thread_rng();
    pairs.shuffle(&mut rng);

    pairs
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn render_board(board: &[Tile]) {
    for (index, tile) in board.iter().enumerate() {
        if index > 0 && index % BOARD_SIZE == 0 {
            println!();
        }
        if tile.visible {
            print!(" {} ", tile.image);
        } else {
            print!(" . ");
        }
    }
    println!();
}

fn get_player_input(board: &[Tile]) -> (usize, usize) {
    loop {
        let first_choice = get_user_choice("Enter the index of the first tile: ", &board);
        let second_choice = get_user_choice("Enter the index of the second tile: ", &board);

        if first_choice != second_choice {
            return (first_choice, second_choice);
        } else {
            println!("You selected the same tile twice. Please choose different tiles.");
        }
    }
}

fn get_user_choice(prompt: &str, board: &[Tile]) -> usize {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(index) => {
                if index < board.len() {
                    if !board[index].visible {
                        println!("Selected tile value: {}", board[index].image);
                        return index;
                    } else {
                        println!("Tile already matched. Choose a different tile.");
                    }
                } else {
                    println!(
                        "Invalid tile index. Please enter a number between 0 and {}.",
                        board.len() - 1
                    );
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}

fn reveal_tiles(board: &mut [Tile], first_choice: usize, second_choice: usize) {
    board[first_choice].visible = true;
    board[second_choice].visible = true;
}

fn cover_tiles(board: &mut [Tile], first_choice: usize, second_choice: usize) {
    board[first_choice].visible = false;
    board[second_choice].visible = false;
}
