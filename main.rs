mod constants;
use std::thread;
use std::time::Duration;
use tokio::spawn;
use console::{Key, Term};

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
//Board will have 7 rows, each 14 tiles wide.
#[derive(Debug)]
struct GameState {
	gameboard: Vec<Row>,
	player: (usize, usize),
	key_reader: KeyReader,
} 

impl GameState {
    async fn run(&mut self) {
		loop {
			print!("\x1B[2J\x1B[1;1H");
			self.formatter();
			if let Some(key) = self.key_reader.read_key().await {
				if key == Key::Escape {
					break;
				}
			}
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
	let rows = vec![Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
	Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
	Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
	Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
	Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
	Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),
	Row::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS),];
	
	let mut game_state = GameState {
		gameboard: rows,
		player: (0, 0),
		key_reader: KeyReader::new(),
	};
	
	game_state.run().await;
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
