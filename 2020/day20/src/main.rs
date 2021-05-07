use std::fs;

use clap::{App, Arg};
use regex::Regex;

const INVALID_ID: i64 = -1;

#[derive(Debug)]
struct Tile {
    id: i64,
    pic: Vec<String>,
}

impl Tile {
    fn new() -> Self {
        Self {
            id: INVALID_ID,
            pic: Vec::new(),
        }
    }

    fn top(&self) -> String {
        self.pic.first().unwrap().to_string()
    }

    fn bottom(&self) -> String {
        self.pic.last().unwrap().to_string()
    }

    // downward
    fn left(&self) -> String {
        let len = self.pic.len();
        let mut string: String = String::with_capacity(len);
        for line in &self.pic {
            string.push(line.chars().collect::<Vec<char>>()[0]);
        }
        string
    }

    fn right(&self) -> String {
        let len = self.pic[0].len();
        let mut string: String = String::with_capacity(len);
        for line in &self.pic {
            string.push(line.chars().collect::<Vec<char>>()[len - 1]);
        }
        string
    }

    // The order is left, right, top, bottom
    fn borders(&self) -> Vec<String> {
        let mut string = Vec::with_capacity(4);
        string.push(self.left());
        string.push(self.right());
        string.push(self.top());
        string.push(self.bottom());
        string
    }
}

fn parse_tile_id(s: &str) -> i64 {
    let re = Regex::new(r"(\d{4})").unwrap();
    let m = re.find(s).unwrap();
    m.as_str().parse::<i64>().unwrap()
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

fn is_original_border(id: i64, string: &String, tiles: &Vec<Tile>) -> bool {
    for tile in tiles {
        if tile.id == id {
            // Skip the same tile
            continue;
        }
        for border in tile.borders() {
            if border.eq(string) {
                return false;
            }
            if border.chars().rev().collect::<String>().eq(string) {
                return false;
            }
        }
    }
    true
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // Read input file specified by command line
    let matches = App::new("2020-20")
        .version("0.1.0")
        .author("Kazuhei Hamada")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the inpust file")
                .required(true)
                .index(1),
        )
        .get_matches();
    let fname = matches.value_of("INPUT").unwrap();
    let input = fs::read_to_string(&fname)?;

    let tiles = parse_tiles(&input);
    let mut corner_ids = Vec::new();
    let mut outer_ids = Vec::new();
    for tile in &tiles {
        let mut originals = Vec::with_capacity(4);
        if is_original_border(tile.id, &tile.left(), &tiles) {
            originals.push("left");
        }
        if is_original_border(tile.id, &tile.right(), &tiles) {
            originals.push("right");
        }
        if is_original_border(tile.id, &tile.top(), &tiles) {
            originals.push("top");
        }
        if is_original_border(tile.id, &tile.bottom(), &tiles) {
            originals.push("bottom");
        }
        println!("ID: {}, originals: {:?}", tile.id, originals);
        if originals.len() == 1 {
            outer_ids.push(tile.id);
        } else if originals.len() == 2 {
            corner_ids.push(tile.id);
        }
    }

    println!(
        "outer_ids.len(): {}, outer_ids: {:?}",
        outer_ids.len(),
        outer_ids
    );
    println!("corner_ids: {:?}", corner_ids);
    println!(
        "Answer is {}",
        corner_ids[0] * corner_ids[1] * corner_ids[2] * corner_ids[3]
    );
    Ok(())
}
