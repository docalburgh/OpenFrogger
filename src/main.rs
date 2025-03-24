mod constants;
use std::thread;
use std::time::Duration;
use tokio::spawn;
use console::{Key, Term};
use rand::Rng;

#[derive(Debug)]
pub struct BaseRow {
	objects: Vec<bool>,
	object_label: char,
	environment_label: char,
}

impl BaseRow {
	pub fn new(objects: Vec<bool>, object_label: char, environment_label: char) -> Self {
		BaseRow {
			objects,
			object_label,
			environment_label,
		}
	}
}
//try modifying this code with formatters logic
impl RowTrait for BaseRow {
	fn display(&self) {
        for &tile in &self.objects {
            print!("{}", if tile { self.object_label } else { self.environment_label });
        }
        println!();
    }

	fn get_objects(&self) -> &Vec<bool> {
		&self.objects
	}

	fn get_char_at(&self, index: usize) -> char {
        if index < self.objects.len() {
            if self.objects[index] {
                self.object_label
            } else {
                self.environment_label
            }
        } else {
            ' ' // Default for out of bounds
        }
    }
}

#[derive(Debug)]
pub struct DynamicRow {
	base: BaseRow,
	tick_count: u8,
	interval: u8,
	direction: bool,
}
//potentially mod this w/old formatter code
impl RowTrait for DynamicRow {
	fn display(&self) {
		for &tile in &self.base.objects {
			print!("{}", if tile { self.base.object_label } else { self.base.environment_label });
        }
        println!();
	}
	fn get_objects(&self) -> &Vec<bool> {
		&self.base.objects
	}

	fn get_char_at(&self, index: usize) -> char {
        if index < self.base.objects.len() {
            if self.base.objects[index] {
                self.base.object_label
            } else {
                self.base.environment_label
            }
        } else {
            ' ' // Default for out of bounds
        }
    }
}

impl DynamicRow {
	pub fn tick(&mut self) {
		//if the row is at tick_count threshold
		//if tick_count <- interval {
		//	self.update_row}
	}

	pub fn update_dynamic_row(&mut self) {
		//perform the dynamic movements of the row horizontally based on direction
	}
}

#[derive(Debug)]
pub struct Stream {
	pub dynamic_row: DynamicRow,
}

impl Stream {
	pub fn new(objects: Vec<bool>, object_label: char, environment_label: char) -> Self {
        let base = BaseRow::new(objects, object_label, environment_label);
        let dynamic_row = DynamicRow {
            base,
            tick_count: 0, //made up number
            interval: 0, //made up number
            direction: true, //bool doesnt mean anything yet, just need code to run first
        };
        Self { dynamic_row: dynamic_row }
    }
}

impl RowTrait for Stream {
	fn display(&self) {
		self.dynamic_row.base.display();
	}

	fn get_objects(&self) -> &Vec<bool> {
        &self.dynamic_row.base.objects
    }

	fn get_char_at(&self, index: usize) -> char {
        if index < self.dynamic_row.base.objects.len() {
            if self.dynamic_row.base.objects[index] {
                self.dynamic_row.base.object_label
            } else {
                self.dynamic_row.base.environment_label
            }
        } else {
            ' ' // Default for out of bounds
        }
    }
}

#[derive(Debug)]
pub struct Road {
	pub dynamic_row: DynamicRow,
}

impl Road {
    pub fn new(objects: Vec<bool>, object_label: char, environment_label: char) -> Self {
        let base = BaseRow::new(objects, object_label, environment_label);
        let dynamic_row = DynamicRow {
            base,
            tick_count: 0,
            interval: 5,
            direction: false,
        };
        Road { dynamic_row }
    }
}

impl RowTrait for Road {
    fn display(&self) {
        self.dynamic_row.base.display();
    }

	fn get_objects(&self) -> &Vec<bool> {
        &self.dynamic_row.base.objects
    }

	fn get_char_at(&self, index: usize) -> char {
        if index < self.dynamic_row.base.objects.len() {
            if self.dynamic_row.base.objects[index] {
                self.dynamic_row.base.object_label
            } else {
                self.dynamic_row.base.environment_label
            }
        } else {
            ' ' // Default for out of bounds
        }
    }
}

pub trait RowTrait: std::fmt::Debug {
	fn display(&self);
	fn get_objects(&self) -> &Vec<bool>;
	fn get_char_at(&self, index: usize) -> char;
}

pub fn create_random_row() -> Box<dyn RowTrait> {
	let objects: Vec<bool> = (0..14).map(|_| rand::rng().random_bool(0.5)).collect();
	
	match rand::rng().random_range(0..3) {
		0 => Box::new(BaseRow::new(objects.clone(), constants::TREE, constants::GRASS)),
		1 => Box::new(Stream::new(objects, constants::LOG, constants::WATER)),
		_ => Box::new(Road::new(objects, constants::CAR, constants::ROAD)),
	}
}

//Board has 7 rows, each 14 tiles wide.
#[derive(Debug)]
struct GameState {
	gameboard: Vec<Box<dyn RowTrait>>,
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
			for col_index in 0..row.get_objects().len() {
				if (row_index, col_index) == self.player {
					print!("{}", constants::PLAYER);
				} else {
					print!("{}", row.get_char_at(col_index));
				}
			}
			println!();
		}
		println!();
		println!("PLAYER SCORE: {}", self.player_score);
    }


	fn new_game() -> Self {
		GameState {
			gameboard: vec![Box::new(BaseRow::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS)),
			Box::new(BaseRow::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS)),
			Box::new(BaseRow::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS)),
			Box::new(BaseRow::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS)),
			Box::new(BaseRow::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS)),
			Box::new(BaseRow::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS)),
			Box::new(BaseRow::new(vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true], constants::TREE, constants::GRASS)),],
			player: (6, 6),
			key_reader: KeyReader::new(),
			player_score: 0,
		}
		
	}

	fn player_movement(&mut self, key: Key) {
		//this captures current player pos before a move
		//this is important for blocking logic
		let current_position = self.player;
		
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
		if self.player.0 <= 1 {
			self.update_stack();
			self.player.0 += 1;
		}
		//create tree blocking logic here
		let row_index = self.player.0;
		let col_index = self.player.1;
		//check if player will move into TREE,
		//if so, return player to current position
		if row_index < self.gameboard.len() {
			let row = &self.gameboard[row_index];
			// Check if there's a tree at this position
			if col_index < row.get_objects().len() && row.get_objects()[col_index] && row.get_char_at(col_index) == constants::TREE {
				// Revert to previous position if there's a tree
				self.player = current_position;
			}
			if col_index < row.get_objects().len() && row.get_objects()[col_index] && row.get_char_at(col_index) == constants::CAR {
				//print ROAD KILL to screen
				println!("ROAD KILL!");
				//pause for 3 seconds before exiting game
				thread::sleep(Duration::from_secs(3));
				std::process::exit(0);
			}
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
		if self.player_position_checker() {
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

	fn player_position_checker(&mut self) -> bool {
		self.player.0 <= 1 
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