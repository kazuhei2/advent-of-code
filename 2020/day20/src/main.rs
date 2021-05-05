use std::fs;

use clap::{Arg, App};
use regex::Regex;

const INVALID_ID: i32 = -1;

struct Tile {
    id: i32,
    pic: Vec<String>,
}

impl Tile {
    fn new() -> Self {
        Self {
            id: INVALID_ID,
            pic: Vec::new(),
        }
    }
}

fn parse_tile_id(s: &str) -> i32 {
    let re = Regex::new(r"(\d{4})").unwrap();
    let m = re.find(s).unwrap();
    m.as_str().parse::<i32>().unwrap()
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    let mut tiles = Vec::new();
    for tile in input.split("\n\n") {
        let mut tmp = Tile::new();
        for line in tile.split("\n") {
            if line.starts_with("Tile") {
                tmp.id = parse_tile_id(line);
            } else {
                tmp.pic.push(line.to_string());
            }
        }
        if tmp.id != INVALID_ID {
            tiles.push(tmp);
        }
    }
    tiles
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let matches = App::new("2020-20")
        .version("0.1.0")
        .author("Kazuhei Hamada")
        .arg(Arg::with_name("INPUT")
            .help("Sets the inpust file")
            .required(true)
            .index(1)
        )
        .get_matches();
    let fname = matches.value_of("INPUT").unwrap();
    let input = fs::read_to_string(&fname)?;

    let tiles = parse_tiles(&input);

    Ok(())
}
