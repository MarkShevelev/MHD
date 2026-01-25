pub struct Uf32<'a>
{
  pub rho: &'a mut [f32],
  pub mnt: &'a mut [f32],
  pub bz : &'a mut [f32],
}

#[derive(Clone, Copy)]
pub struct Uf32View<'a>
{
  pub rho: &'a [f32],
  pub mnt: &'a [f32],
  pub bz : &'a [f32],
}

impl<'a> Uf32<'a>
{
  pub fn new (
    rho: &'a mut [f32], mnt: &'a mut [f32], bz: &'a mut [f32]) -> Self
  {
    Self { rho, mnt, bz }
  }

  pub fn new_with_slice(
    rho: &'a mut [f32], mnt: &'a mut [f32], bz: &'a mut [f32],
    begin: usize, end: usize) -> Self
  {
    Self {
      rho: &mut rho[begin..end],
      mnt: &mut mnt[begin..end],
      bz:  &mut bz [begin..end]
    }
  }
}

impl<'a> Uf32View<'a>
{
  pub fn new (
    rho: &'a [f32], mnt: &'a [f32], bz: &'a [f32]) -> Self
  {
    Self { rho, mnt, bz }
  }

  pub fn new_with_slice(
    rho: &'a [f32], mnt: &'a [f32], bz: &'a [f32],
    begin: usize, end: usize) -> Self
  {
    Self {
      rho: &rho[begin..end],
      mnt: &mnt[begin..end],
      bz:  &bz [begin..end]
    }
  }
}

impl<'a, 'b> From<&'a Uf32<'b>> for Uf32View<'a> {
  fn from(src: &'a Uf32<'b>) -> Self {
    Uf32View {
      rho: src.rho,
      mnt: src.mnt,
      bz:  src.bz,
    }
  }
}

pub struct U2df32<'a>
{
  pub rho: &'a mut [f32],
  pub mnt: &'a mut [f32],
  pub bz : &'a mut [f32],
  pub rows: usize,
  pub cols: usize,
}

impl<'a> U2df32<'a>
{
  pub fn new (
    rho: &'a mut [f32], mnt: &'a mut [f32], bz: &'a mut [f32],
    rows: usize, cols: usize) -> Self
  {
    Self { rho, mnt, bz, rows, cols }
  }
}

#[derive(Clone, Copy)]
pub struct U2dViewf32<'a>
{
  pub rho: &'a [f32],
  pub mnt: &'a [f32],
  pub bz : &'a [f32],
  pub rows: usize,
  pub cols: usize,
}

impl<'a> U2dViewf32<'a>
{
  pub fn new (
    rho: &'a [f32], mnt: &'a [f32], bz: &'a [f32],
    rows: usize, cols: usize) -> Self
  {
    Self { rho, mnt, bz, rows, cols }
  }
}

impl<'a, 'b> From<&'a U2df32<'b>> for U2dViewf32<'a> {
  fn from(src: &'a U2df32<'b>) -> Self {
    U2dViewf32 {
      rho: src.rho,
      mnt: src.mnt,
      bz:  src.bz,
      rows: src.rows,
      cols: src.cols,
    }
  }
}
