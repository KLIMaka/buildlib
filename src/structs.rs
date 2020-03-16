use bitfield::bitfield;

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

#[derive(Debug)]
pub struct Sector {
  pub wallptr: u16,
  pub wallnum: u16,
  pub ceilingz: i32,
  pub floorz: i32,
  pub ceilingstat: SectorStats,
  pub floorstat: SectorStats,
  pub ceilingpicnum: u16,
  pub ceilingheinum: i16,
  pub ceilingshade: i8,
  pub ceilingpal: u8,
  pub ceilingxpanning: u8,
  pub ceilingypanning: u8,
  pub floorpicnum: u16,
  pub floorheinum: i16,
  pub floorshade: i8,
  pub floorpal: u8,
  pub floorxpanning: u8,
  pub floorypanning: u8,
  pub visibility: i8,
  pub filler: i8,
  pub lotag: u16,
  pub hitag: u16,
  pub extra: u16,
}

#[derive(Debug)]
pub struct Wall {
  pub x: i32,
  pub y: i32,
  pub point2: u16,
  pub nextwall: i16,
  pub nextsector: i16,
  pub cstat: WallStats,
  pub picnum: u16,
  pub overpicnum: u16,
  pub shade: i8,
  pub pal: u8,
  pub xrepeat: u8,
  pub yrepeat: u8,
  pub xpanning: u8,
  pub ypanning: u8,
  pub lotag: u16,
  pub hitag: u16,
  pub extra: u16,
}

#[derive(Debug)]
pub struct Sprite {
  pub x: i32,
  pub y: i32,
  pub z: i32,
  pub cstat: SpriteStats,
  pub picnum: u16,
  pub shade: i8,
  pub pal: u8,
  pub clipdist: u8,
  pub filler: u8,
  pub xrepeat: u8,
  pub yrepeat: u8,
  pub xoffset: u8,
  pub yoffset: u8,
  pub sectnum: u16,
  pub statnum: u16,
  pub ang: u16,
  pub owner: u16,
  pub xvel: i16,
  pub yvel: i16,
  pub zvel: i16,
  pub lotag: u16,
  pub hitag: u16,
  pub extra: u16,
}

#[derive(Debug)]
pub struct Header {
  version: u32,
  posx: i32,
  posy: i32,
  posz: i32,
  ang: u16,
  cursectnum: u16,
}

pub struct Board {
  pub header: Header,
  pub sectors: Vec<Sector>,
  pub walls: Vec<Wall>,
  pub sprites: Vec<Sprite>,
}

use byteorder::{LittleEndian, ReadBytesExt};
use std::io;
use std::io::Read;

impl SectorStats {
  pub fn read(r: &mut impl Read) -> io::Result<Self> {
    Ok(SectorStats(r.read_u16::<LittleEndian>()?))
  }
}

impl Sector {
  pub fn read(r: &mut impl Read) -> io::Result<Self> {
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

impl WallStats {
  pub fn read(r: &mut impl Read) -> io::Result<Self> {
    Ok(WallStats(r.read_u16::<LittleEndian>()?))
  }
}

impl Wall {
  pub fn read(r: &mut impl Read) -> io::Result<Self> {
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

impl SpriteStats {
  pub fn read(r: &mut impl Read) -> io::Result<Self> {
    let data = r.read_u16::<LittleEndian>()?;
    Ok(SpriteStats(data))
  }
}

impl Sprite {
  pub fn read(r: &mut impl Read) -> io::Result<Self> {
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

impl Board {
  pub fn read(r: &mut impl Read) -> io::Result<Self> {
    let header = Header::read(r)?;
    let numsectors = r.read_u16::<LittleEndian>()?;
    let mut sectors = Vec::new();
    for _ in 0..numsectors {
      sectors.push(Sector::read(r)?);
    }
    let numwalls = r.read_i16::<LittleEndian>()?;
    let mut walls = Vec::new();
    for _ in 0..numwalls {
      walls.push(Wall::read(r)?);
    }
    let numsprites = r.read_u16::<LittleEndian>()?;
    let mut sprites = Vec::new();
    for _ in 0..numsprites {
      sprites.push(Sprite::read(r)?);
    }

    Ok(Board { header, sectors, walls, sprites })
  }

  pub fn walllen(self: &Board, wall_id: usize) -> f32 {
    let wall1 = &self.walls[wall_id];
    let wall2 = &self.walls[wall1.point2 as usize];

    (((wall1.x - wall2.x).pow(2) + (wall1.y - wall2.y).pow(2)) as f32).sqrt()
  }
}
