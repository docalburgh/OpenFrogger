mod constants;

#[derive(Debug)]
pub struct Row {
	objects: Vec<bool>,
	object_label: char,
	environment_label: char,
}

impl Row {
	pub fn new(objects: Vec<bool>, object_label: char, environment_label: char) -> Self {
		Row {
			objects,
			object_label,
			environment_label,
		}
	}
}
//consider making gameboard a vec of Row(s)
//Board will have 7 rows, each 14 tiles wide.
#[derive(Debug)]
struct GameState {
	gameboard: Vec<Row>,
	player: (usize, usize),
} 

impl GameState {
    fn formatter(&self) {
        // Iterate over each row with its index.
        for (row_index, row) in self.gameboard.iter().enumerate() {
            // For each row, iterate over each column.
            for (col_index, &tile) in row.objects.iter().enumerate() {
                // If the current coordinates match the player's, print the player symbol.
                if (row_index, col_index) == self.player {
                    print!("{}", constants::PLAYER);
                } else if tile {
                    // If the boolean is true, print the object's label.
                    print!("{}", row.object_label);
                } else {
                    // Otherwise, print the environment label.
                    print!("{}", row.environment_label);
                }
            }
            // Move to the next line after finishing a row.
            println!();
        }
    }
}

fn main() {
	let rows = vec![Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
	Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
	Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
	Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
	Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
	Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
	Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),];
	
	let game_state = GameState {
		gameboard: rows,
		player: (0, 0),
	};
	
	game_state.formatter();
}

/* impl Row {
	pub fn new(label: char) -> Self {
	}
}

struct GameState {
	gameboard: Row,
	player: usize //eventually going to be a tuple of coordinates x,y
}

impl GameState {
	pub fn new_game() -> Self {
	}

	pub async fn run(&mut self) -> {
		loop {
			self.print_grid();
		}
	}

	pub print_grind() {
		println!("This is the game");
		//display the game logic to a player in a way that makes sense
	}
}

fn main() {
	let mut game_state = GameState.new_game();
	game_state.run().await;
} */
