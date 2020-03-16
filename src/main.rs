mod structs;
use std::fs::File;
use std::io;
use structs::*;

fn main() -> io::Result<()> {
  let mut f = File::open("1.map")?;
  let board = Board::read(&mut f)?;
  println!("{:?}", board.header);
  println!(
    "sectors: {} walls: {} sprites: {}",
    board.sectors.len(),
    board.walls.len(),
    board.sprites.len()
  );

  println!("wall 1 length {}", board.walllen(554));

  Ok(())
}
