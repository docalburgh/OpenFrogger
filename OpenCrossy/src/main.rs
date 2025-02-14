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

#[derive(Debug)]
struct GameState {
	gameboard: Row,
	player: usize //eventually going to be a tuple of coordinates x,y
}

impl GameState {
	fn formatter(&self) {
		for (index, &boolean) in self.gameboard.objects.iter().enumerate() {
			if index == self.player {
				print!("{}", constants::PLAYER);
			} else if boolean {
				print!("{}", &self.gameboard.object_label);
			} else {
				print!("{}", &self.gameboard.environment_label);
			}
		}
	}
}

fn main() {
	let row1 = Row::new(vec![true, false, true, false, false], constants::TREE, constants::GRASS);
	let game_state = GameState {
		gameboard: row1,
		player: 0,
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
