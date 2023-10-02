use std::io;

const SIZE: usize = 4;

fn main() {
    let mut puzzle = create_puzzle();
    let mut moves = 0;
    
    loop {
        print_puzzle(&puzzle);
        if is_solved(&puzzle) {
            println!("Congratulations! You solved the puzzle in {} moves.", moves);
            break;
        }
        
        println!("Enter your move (WASD or Q to quit): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let input = input.trim().to_uppercase();
        if input == "Q" {
            println!("Quitting the game.");
            break;
        }
        
        let direction = match input.as_str() {
            "W" => MoveDirection::Up,
            "A" => MoveDirection::Left,
            "S" => MoveDirection::Down,
            "D" => MoveDirection::Right,
            _ => {
                println!("Invalid move. Use WASD to move or Q to quit.");
                continue;
            }
        };
        
        if !make_move(&mut puzzle, direction) {
            println!("Invalid move. Try again.");
        } else {
            moves += 1;
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

fn create_puzzle() -> [[u8; SIZE]; SIZE] {
    let mut puzzle = [[0; SIZE]; SIZE];
    let mut num = 1;
    
    for row in 0..SIZE {
        for col in 0..SIZE {
            puzzle[row][col] = num;
            num += 1;
        }
    }
    
    puzzle[SIZE - 1][SIZE - 1] = 0;
    shuffle_puzzle(&mut puzzle);
    
    puzzle
}

fn shuffle_puzzle(puzzle: &mut [[u8; SIZE]; SIZE]) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();
    let directions = [MoveDirection::Up, MoveDirection::Down, MoveDirection::Left, MoveDirection::Right];

    for _ in 0..1000 {
        let random_direction = *directions.choose(&mut rng).unwrap();
        make_move(puzzle, random_direction);
    }
}

fn make_move(puzzle: &mut [[u8; SIZE]; SIZE], direction: MoveDirection) -> bool {
    let (empty_row, empty_col) = find_empty_square(puzzle);
    let (new_row, new_col) = match direction {
        MoveDirection::Up => (empty_row - 1, empty_col),
        MoveDirection::Down => (empty_row + 1, empty_col),
        MoveDirection::Left => (empty_row, empty_col - 1),
        MoveDirection::Right => (empty_row, empty_col + 1),
    };

    if new_row < 0 || new_row >= SIZE as i32 || new_col < 0 || new_col >= SIZE as i32 {
        return false;
    }

    puzzle[empty_row as usize][empty_col as usize] = puzzle[new_row as usize][new_col as usize];
    puzzle[new_row as usize][new_col as usize] = 0;
    true
}

fn find_empty_square(puzzle: &[[u8; SIZE]; SIZE]) -> (i32, i32) {
    for row in 0..SIZE as i32 {
        for col in 0..SIZE as i32 {
            if puzzle[row as usize][col as usize] == 0 {
                return (row, col);
            }
        }
    }
    unreachable!()
}

fn is_solved(puzzle: &[[u8; SIZE]; SIZE]) -> bool {
    let mut num = 1;
    for row in 0..SIZE {
        for col in 0..SIZE {
            if puzzle[row][col] != num {
                return false;
            }
            num += 1;
        }
    }
    true
}

fn print_puzzle(puzzle: &[[u8; SIZE]; SIZE]) {
    for row in puzzle.iter() {
        for &cell in row.iter() {
            if cell == 0 {
                print!("   ");
            } else {
                print!("{:2} ", cell);
            }
        }
        println!();
    }
    println!();
}
