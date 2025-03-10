mod constants;
use std::thread;
use std::time::Duration;
use tokio::spawn;
use console::{Key, Term};
use rand::Rng;

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

//standalone function
pub fn create_random_row() -> Row {
	let objects: Vec<bool> = (0..15).map(|_| rand::rng().random_bool(0.5)).collect();
	Row::new(objects, constants::GRASS, constants::TREE)
}

//Board has 7 rows, each 14 tiles wide.
#[derive(Debug)]
struct GameState {
	gameboard: Vec<Row>,
	player: (usize, usize),
	key_reader: KeyReader,
	player_score: u16, //u16 means max score is 65,535
} 

impl GameState {
    async fn run(&mut self) {
		loop {
			print!("\x1B[2J\x1B[1;1H");
			self.formatter();
			self.key_reader_catchall().await;
			thread::sleep(Duration::from_millis(50));
		}
	}

	fn formatter(&self) {
        for (row_index, row) in self.gameboard.iter().enumerate() {
            for (col_index, &tile) in row.objects.iter().enumerate() {
                if (row_index, col_index) == self.player {
                    print!("{}", constants::PLAYER);
                } else if tile {
                    print!("{}", row.object_label);
                } else {
                    print!("{}", row.environment_label);
                }
            }
            println!();
        }
    }

	fn new_game() -> Self {
		GameState {
			gameboard: vec![Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
			Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
			Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
			Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
			Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
			Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
			Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),],
			player: (6, 7),
			key_reader: KeyReader::new(),
			player_score: 0,
		}
		
	}

	fn player_movement(&mut self, key: Key) {
		match key {
				Key::Char('w') => if self.player.0 > 0
					{self.player.0 -= 1},
				Key::Char('a') => if self.player.1 > 0 
					{self.player.1 -= 1},
				Key::Char('s') => if self.player.0 < 6
					{self.player.0 += 1},
				Key::Char('d') => if self.player.1 < 13
					{self.player.1 += 1},
				Key::Escape => std::process::exit(0),
				_ => (),
			
		}
	}

	async fn key_reader_catchall(&mut self) {
		if let Some(key) = self.key_reader.read_key().await {
			match key {
				Key::Char('w') | Key::Char('a') | Key::Char('s') | Key::Char('d') =>
				self.player_movement(key),
				_ => self.misc_key(key),
			}
		}
	}

	fn misc_key(&mut self, key: Key) {
		match key {
			Key::Escape => std::process::exit(0),
			_ => (),
		}
	}

	fn update_stack(&mut self) {
		//run create_random_row
		let new_row = create_random_row();

		//insert new random row at beginning of the gameboard vector
		self.gameboard.insert(0, new_row);

		//remove bottom most row from memory
		self.gameboard.pop();

		//increment player score
		self.player_score +=1;
	}
}

#[derive(Debug)]
pub struct KeyReader {
    jh: Option<tokio::task::JoinHandle<Key>>,
}

impl KeyReader {
    pub fn new() -> KeyReader {
        KeyReader {
            jh: Some(tokio::spawn(Self::await_key_press())),
        }
    }
    async fn await_key_press() -> Key {
        let term = Term::stdout();
        term.read_key().unwrap()
    }
    pub async fn read_key(&mut self) -> Option<Key> {
        if self.jh.as_ref().unwrap().is_finished() {
            let key = self.jh.take().unwrap().await.unwrap();
            self.jh = Some(tokio::spawn(Self::await_key_press()));
            Some(key)
        } else {
            None
        }
    }
}


#[tokio::main]
async fn main() {
	let mut game_state = GameState::new_game();
	game_state.run().await;
}