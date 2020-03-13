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
    let data = r.read_u16::<LittleEndian>()?;
    Ok(SectorStats(data))
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
    let wallptr = r.read_u16::<LittleEndian>()?;
    let wallnum = r.read_u16::<LittleEndian>()?;
    let ceilingz = r.read_i32::<LittleEndian>()?;
    let floorz = r.read_i32::<LittleEndian>()?;
    let ceilingstat = SectorStats::read(r)?;
    let floorstat = SectorStats::read(r)?;
    let ceilingpicnum = r.read_u16::<LittleEndian>()?;
    let ceilingheinum = r.read_i16::<LittleEndian>()?;
    let ceilingshade = r.read_i8()?;
    let ceilingpal = r.read_u8()?;
    let ceilingxpanning = r.read_u8()?;
    let ceilingypanning = r.read_u8()?;
    let floorpicnum = r.read_u16::<LittleEndian>()?;
    let floorheinum = r.read_i16::<LittleEndian>()?;
    let floorshade = r.read_i8()?;
    let floorpal = r.read_u8()?;
    let floorxpanning = r.read_u8()?;
    let floorypanning = r.read_u8()?;
    let visibility = r.read_i8()?;
    let filler = r.read_i8()?;
    let lotag = r.read_u16::<LittleEndian>()?;
    let hitag = r.read_u16::<LittleEndian>()?;
    let extra = r.read_u16::<LittleEndian>()?;

    Ok(Sector {
      wallptr,
      wallnum,
      ceilingz,
      floorz,
      ceilingstat,
      floorstat,
      ceilingpicnum,
      ceilingheinum,
      ceilingshade,
      ceilingpal,
      ceilingxpanning,
      ceilingypanning,
      floorpicnum,
      floorheinum,
      floorshade,
      floorpal,
      floorxpanning,
      floorypanning,
      visibility,
      filler,
      lotag,
      hitag,
      extra,
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
    let version = r.read_u32::<LittleEndian>()?;
    let posx = r.read_i32::<LittleEndian>()?;
    let posy = r.read_i32::<LittleEndian>()?;
    let posz = r.read_i32::<LittleEndian>()?;
    let ang = r.read_u16::<LittleEndian>()?;
    let cursectnum = r.read_u16::<LittleEndian>()?;

    Ok(Header {
      version,
      posx,
      posy,
      posz,
      ang,
      cursectnum,
    })
  }
}

use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
  let mut f = File::open("1.map")?;
  let header = Header::read(&mut f)?;
  println!("header {:?}", header);
  let numsectors = f.read_u16::<LittleEndian>()?;
  let mut sectors = Vec::new();
  for _ in 0..numsectors {
    sectors.push(Sector::read(&mut f)?);
  }
  Ok(())
}
