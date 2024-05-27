use std::io::{Read, Error, self};
use std::fs::File;
use std::process;

use crate::game::Map;
use crate::player::{Player, Vector};

fn open_conf_file(args: Vec<String>) -> io::Result<String> {
    if args.len() != 2 {
        eprintln!("Give 1 (and only 1!) Argument please!\n");
        process::exit(1);
    };
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub struct Config {
    pub player_start: Player,
    pub map: Map

}

impl Config {
    pub fn parse(args: Vec<String>) -> Result<Self, Error> {
        let constFile = open_conf_file(args).expect("Problems with Reading File!");

        println!("This is the Const File!\n{}", constFile);


        let position: Vector = Vector {x: 2.0, y: 3.0};
        let direction: Vector = Vector {x: 0.0, y: 5.0};

        Ok(Config {
            player_start: Player::new(position, direction),
            map: Map::new(),
        })
    }
}
