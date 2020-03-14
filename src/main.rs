#[macro_use]
extern crate bitfield;

bitfield! {
  #[derive(Debug)]
  pub struct SectorStats(u16);
  pub parallaxing, set_parallaxing: 1, 0;
  pub slopped, set_slopped: 2, 1;
  pub swap, set_swap: 3, 2;
  pub double, set_double: 4, 3;
  pub xflip, set_xflip: 5, 4;
  pub yflip, set_yflip: 6, 5;
  pub align, set_align: 7, 6;
  pub unk, set_unk: 15, 7;
}

impl SectorStats {
  fn read(r: &mut impl Read) -> io::Result<Self> {
    Ok(SectorStats(r.read_u16::<LittleEndian>()?))
  }
}

#[derive(Debug)]
struct Sector {
  wallptr: u16,
  wallnum: u16,
  ceilingz: i32,
  floorz: i32,
  ceilingstat: SectorStats,
  floorstat: SectorStats,
  ceilingpicnum: u16,
  ceilingheinum: i16,
  ceilingshade: i8,
  ceilingpal: u8,
  ceilingxpanning: u8,
  ceilingypanning: u8,
  floorpicnum: u16,
  floorheinum: i16,
  floorshade: i8,
  floorpal: u8,
  floorxpanning: u8,
  floorypanning: u8,
  visibility: i8,
  filler: i8,
  lotag: u16,
  hitag: u16,
  extra: u16,
}

impl Sector {
  fn read(r: &mut impl Read) -> io::Result<Self> {
    Ok(Sector {
      wallptr: r.read_u16::<LittleEndian>()?,
      wallnum: r.read_u16::<LittleEndian>()?,
      ceilingz: r.read_i32::<LittleEndian>()?,
      floorz: r.read_i32::<LittleEndian>()?,
      ceilingstat: SectorStats::read(r)?,
      floorstat: SectorStats::read(r)?,
      ceilingpicnum: r.read_u16::<LittleEndian>()?,
      ceilingheinum: r.read_i16::<LittleEndian>()?,
      ceilingshade: r.read_i8()?,
      ceilingpal: r.read_u8()?,
      ceilingxpanning: r.read_u8()?,
      ceilingypanning: r.read_u8()?,
      floorpicnum: r.read_u16::<LittleEndian>()?,
      floorheinum: r.read_i16::<LittleEndian>()?,
      floorshade: r.read_i8()?,
      floorpal: r.read_u8()?,
      floorxpanning: r.read_u8()?,
      floorypanning: r.read_u8()?,
      visibility: r.read_i8()?,
      filler: r.read_i8()?,
      lotag: r.read_u16::<LittleEndian>()?,
      hitag: r.read_u16::<LittleEndian>()?,
      extra: r.read_u16::<LittleEndian>()?,
    })
  }
}

bitfield! {
  #[derive(Debug)]
  pub struct WallStats(u16);
  pub blocking, set_blocking: 1, 0;
  pub swap, set_swap: 2, 1;
  pub align, set_align: 3, 2;
  pub xflip, set_xflip: 4, 3;
  pub masking, set_masking: 5, 4;
  pub one, set_one: 6,5;
  pub blocking2, set_blocking2: 7, 6;
  pub translucent, set_translucent: 8, 7;
  pub yflip, set_yflip: 9, 8;
  pub translucent_reversed, set_translucent_reversed: 10, 9;
  pub unk, set_unk: 15, 10;
}

impl WallStats {
  fn read(r: &mut impl Read) -> io::Result<Self> {
    Ok(WallStats(r.read_u16::<LittleEndian>()?))
  }
}

#[derive(Debug)]
struct Wall {
  x: i32,
  y: i32,
  point2: u16,
  nextwall: i16,
  nextsector: i16,
  cstat: WallStats,
  picnum: u16,
  overpicnum: u16,
  shade: i8,
  pal: u8,
  xrepeat: u8,
  yrepeat: u8,
  xpanning: u8,
  ypanning: u8,
  lotag: u16,
  hitag: u16,
  extra: u16,
}

impl Wall {
  fn read(r: &mut impl Read) -> io::Result<Self> {
    Ok(Wall {
      x: r.read_i32::<LittleEndian>()?,
      y: r.read_i32::<LittleEndian>()?,
      point2: r.read_u16::<LittleEndian>()?,
      nextwall: r.read_i16::<LittleEndian>()?,
      nextsector: r.read_i16::<LittleEndian>()?,
      cstat: WallStats::read(r)?,
      picnum: r.read_u16::<LittleEndian>()?,
      overpicnum: r.read_u16::<LittleEndian>()?,
      shade: r.read_i8()?,
      pal: r.read_u8()?,
      xrepeat: r.read_u8()?,
      yrepeat: r.read_u8()?,
      xpanning: r.read_u8()?,
      ypanning: r.read_u8()?,
      lotag: r.read_u16::<LittleEndian>()?,
      hitag: r.read_u16::<LittleEndian>()?,
      extra: r.read_u16::<LittleEndian>()?,
    })
  }
}

bitfield! {
  #[derive(Debug)]
  pub struct SpriteStats(u16);
  pub blocking, set_blocking: 1 ,0;
  pub translucent, set_translucent: 2, 1;
  pub xflip, set_xflip: 3, 2;
  pub yflip, set_yflip: 4, 3;
  pub sprite_type, set_sprite_type: 5, 4;
  pub onesided, set_onesided: 6, 5;
  pub real_center, set_real_center: 7, 6;
  pub blocking2, set_blocking2: 8, 7;
  pub reverse_tranclucent, set_reverse_tranclucent: 9, 8;
  pub noautoshading, set_noautoshading: 10, 9;
  pub unk, set_unk: 14, 10;
  pub invisible, set_invisible: 15, 14;
}

impl SpriteStats {
  fn read(r: &mut impl Read) -> io::Result<Self> {
    let data = r.read_u16::<LittleEndian>()?;
    Ok(SpriteStats(data))
  }
}

#[derive(Debug)]
struct Sprite {
  x: i32,
  y: i32,
  z: i32,
  cstat: SpriteStats,
  picnum: u16,
  shade: i8,
  pal: u8,
  clipdist: u8,
  filler: u8,
  xrepeat: u8,
  yrepeat: u8,
  xoffset: u8,
  yoffset: u8,
  sectnum: u16,
  statnum: u16,
  ang: u16,
  owner: u16,
  xvel: i16,
  yvel: i16,
  zvel: i16,
  lotag: u16,
  hitag: u16,
  extra: u16,
}

impl Sprite {
  fn read(r: &mut impl Read) -> io::Result<Self> {
    Ok(Sprite {
      x: r.read_i32::<LittleEndian>()?,
      y: r.read_i32::<LittleEndian>()?,
      z: r.read_i32::<LittleEndian>()?,
      cstat: SpriteStats::read(r)?,
      picnum: r.read_u16::<LittleEndian>()?,
      shade: r.read_i8()?,
      pal: r.read_u8()?,
      clipdist: r.read_u8()?,
      filler: r.read_u8()?,
      xrepeat: r.read_u8()?,
      yrepeat: r.read_u8()?,
      xoffset: r.read_u8()?,
      yoffset: r.read_u8()?,
      sectnum: r.read_u16::<LittleEndian>()?,
      statnum: r.read_u16::<LittleEndian>()?,
      ang: r.read_u16::<LittleEndian>()?,
      owner: r.read_u16::<LittleEndian>()?,
      xvel: r.read_i16::<LittleEndian>()?,
      yvel: r.read_i16::<LittleEndian>()?,
      zvel: r.read_i16::<LittleEndian>()?,
      lotag: r.read_u16::<LittleEndian>()?,
      hitag: r.read_u16::<LittleEndian>()?,
      extra: r.read_u16::<LittleEndian>()?,
    })
  }
}

#[derive(Debug)]
struct Header {
  version: u32,
  posx: i32,
  posy: i32,
  posz: i32,
  ang: u16,
  cursectnum: u16,
}

impl Header {
  fn read(r: &mut impl Read) -> io::Result<Self> {
    Ok(Header {
      version: r.read_u32::<LittleEndian>()?,
      posx: r.read_i32::<LittleEndian>()?,
      posy: r.read_i32::<LittleEndian>()?,
      posz: r.read_i32::<LittleEndian>()?,
      ang: r.read_u16::<LittleEndian>()?,
      cursectnum: r.read_u16::<LittleEndian>()?,
    })
  }
}

struct Board {
  sectors: Vec<Sector>,
  walls: Vec<Wall>,
  sprites: Vec<Sprite>,
}

impl Board {
  fn walllen(self: &Board, wall_id: usize) -> f32 {
    let wall1 = &self.walls[wall_id];
    let wall2 = &self.walls[wall1.point2 as usize];
    return (((wall1.x - wall2.x).pow(2) + (wall1.y - wall2.y).pow(2)) as f32).sqrt();
  }
}

use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
  let mut f = File::open("1.map")?;
  let header = Header::read(&mut f)?;
  println!("{:?}", header);
  let numsectors = f.read_u16::<LittleEndian>()?;
  let mut sectors = Vec::new();
  for _ in 0..numsectors {
    sectors.push(Sector::read(&mut f)?);
  }

  let numwalls = f.read_i16::<LittleEndian>()?;
  let mut walls = Vec::new();
  for _ in 0..numwalls {
    walls.push(Wall::read(&mut f)?);
  }

  let numsprites = f.read_u16::<LittleEndian>()?;
  let mut sprites = Vec::new();
  for _ in 0..numsprites {
    sprites.push(Sprite::read(&mut f)?);
  }

  let board = Board { sectors, walls, sprites };
  println!(
    "sectors: {} walls: {} sprites: {}",
    board.sectors.len(),
    board.walls.len(),
    board.sprites.len()
  );

  println!("wall 1 length {}", board.walllen(554));

  Ok(())
}
