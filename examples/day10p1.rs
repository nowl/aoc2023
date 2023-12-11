use debug_print::debug_println;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use anyhow::Error;
use itertools::Itertools;
use nom::{character::complete::*, multi::many1, sequence::*, IResult};

lazy_static! {
    static ref NORTH_TILES: Vec<Tile> = vec![
        Tile::NorthSouth,
        Tile::Start,
        Tile::SouthWest,
        Tile::SouthEast
    ];
    static ref SOUTH_TILES: Vec<Tile> = vec![
        Tile::NorthSouth,
        Tile::Start,
        Tile::NorthEast,
        Tile::NorthWest,
    ];
    static ref EAST_TILES: Vec<Tile> = vec![
        Tile::EastWest,
        Tile::Start,
        Tile::NorthWest,
        Tile::SouthWest
    ];
    static ref WEST_TILES: Vec<Tile> = vec![
        Tile::EastWest,
        Tile::Start,
        Tile::NorthEast,
        Tile::SouthEast
    ];
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Debug, Eq, PartialEq, Hash, Default, Clone)]
struct Spot {
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct Data {
    tiles: HashMap<Spot, Tile>,
    start: Spot,
}

fn parse_tile(i: &str) -> IResult<&str, Tile> {
    use Tile::*;

    let (i, c) = one_of("|-LJ7F.S")(i)?;
    let tile = match c {
        '|' => NorthSouth,
        '-' => EastWest,
        'L' => NorthEast,
        'J' => NorthWest,
        '7' => SouthWest,
        'F' => SouthEast,
        '.' => Ground,
        'S' => Start,
        _ => unreachable!(),
    };

    Ok((i, tile))
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = terminated(many1(parse_tile), multispace0);
    let (i, lines) = many1(parse_line)(i)?;
    let (tiles, start) = {
        let mut start = Spot::default();
        let mut map = HashMap::new();
        lines.into_iter().enumerate().for_each(|(row, line)| {
            line.into_iter().enumerate().for_each(|(col, tile)| {
                let spot = Spot {
                    row: row as i32,
                    col: col as i32,
                };
                if tile == Tile::Start {
                    start = spot.clone();
                }

                map.insert(spot, tile);
            });
        });
        (map, start)
    };
    let data = Data { tiles, start };
    Ok((i, data))
}

fn adj_of(data: &Data, spot @ &Spot { row, col }: &Spot) -> Vec<(Spot, Tile)> {
    let mut adj = vec![];

    debug_println!("adjacent to: {spot:?}");

    macro_rules! tile_test {
        ($dir_tiles:ident, $row:expr, $col:expr) => {
            let test_spot = Spot {
                row: $row,
                col: $col,
            };
            if let Some(tile) = data.tiles.get(&test_spot) {
                if $dir_tiles.contains(tile) {
                    adj.push((test_spot.clone(), tile.clone()));
                }
            }
        };
    }

    if *spot == data.start {
        // start
        tile_test!(NORTH_TILES, row - 1, col);
        tile_test!(SOUTH_TILES, row + 1, col);
        tile_test!(EAST_TILES, row, col + 1);
        tile_test!(WEST_TILES, row, col - 1);
    } else {
        match data.tiles.get(spot).unwrap() {
            Tile::NorthSouth => {
                tile_test!(NORTH_TILES, row - 1, col);
                tile_test!(SOUTH_TILES, row + 1, col);
            }
            Tile::EastWest => {
                tile_test!(WEST_TILES, row, col - 1);
                tile_test!(EAST_TILES, row, col + 1);
            }
            Tile::NorthEast => {
                tile_test!(NORTH_TILES, row - 1, col);
                tile_test!(EAST_TILES, row, col + 1);
            }
            Tile::NorthWest => {
                tile_test!(NORTH_TILES, row - 1, col);
                tile_test!(WEST_TILES, row, col - 1);
            }
            Tile::SouthWest => {
                tile_test!(WEST_TILES, row, col - 1);
                tile_test!(SOUTH_TILES, row + 1, col);
            }
            Tile::SouthEast => {
                tile_test!(EAST_TILES, row, col + 1);
                tile_test!(SOUTH_TILES, row + 1, col);
            }
            _ => unreachable!(),
        }
    }

    debug_println!("adjacencies: {adj:?}");

    adj
}

fn follow_path(data: &Data) -> Vec<Spot> {
    let mut path = vec![];
    let mut path_set = HashSet::new();

    let mut pos = data.start.clone();
    path.push(pos.clone());
    path_set.insert(pos.clone());
    loop {
        let adjs = adj_of(data, &pos);
        assert_eq!(adjs.len(), 2);
        if !path_set.contains(&adjs[0].0) {
            pos = adjs[0].0.clone();
        } else {
            pos = adjs[1].0.clone();
        }
        path.push(pos.clone());
        path_set.insert(pos.clone());

        if path.len() > 3 && adjs.iter().map(|(spot, _)| spot).contains(&data.start) {
            break;
        }
    }

    path
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d10p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    let path = follow_path(&data);

    debug_println!("{path:?}");

    let answer = (path.len() - 1) / 2;

    println!("{answer:?}");

    Ok(())
}
