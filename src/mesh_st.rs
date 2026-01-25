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

#[derive(Clone, Copy)]
pub struct U2dCfg
{
  pub total_rows  : usize,
  pub total_cols  : usize,
  pub offset_rows : usize,
  pub offset_cols : usize,
  pub process_rows: usize,
  pub process_cols: usize,
}

#[inline]
pub fn cfg_transpose(cfg: U2dCfg) -> U2dCfg
{
  U2dCfg {
    total_rows  : cfg.total_cols,
    total_cols  : cfg.total_rows,
    offset_rows : cfg.offset_cols,
    offset_cols : cfg.offset_rows,
    process_rows: cfg.process_cols,
    process_cols: cfg.process_rows,
  }
}

pub struct U2df32<'a>
{
  pub rho: &'a mut [f32],
  pub mnt: &'a mut [f32],
  pub bz : &'a mut [f32],
  pub cfg: U2dCfg,
}

impl<'a> U2df32<'a>
{
  pub fn new (
    rho: &'a mut [f32], mnt: &'a mut [f32], bz: &'a mut [f32],
    cfg: U2dCfg) -> Self
  {
    Self { rho, mnt, bz, cfg }
  }
}

#[derive(Clone, Copy)]
pub struct U2dViewf32<'a>
{
  pub rho: &'a [f32],
  pub mnt: &'a [f32],
  pub bz : &'a [f32],
  pub cfg: U2dCfg,
}

impl<'a> U2dViewf32<'a>
{
  pub fn new (
    rho: &'a [f32], mnt: &'a [f32], bz: &'a [f32],
    cfg: U2dCfg) -> Self
  {
    Self { rho, mnt, bz, cfg }
  }
}

impl<'a, 'b> From<&'a U2df32<'b>> for U2dViewf32<'a> {
  fn from(src: &'a U2df32<'b>) -> Self {
    U2dViewf32 {
      rho: src.rho,
      mnt: src.mnt,
      bz:  src.bz,
      cfg: src.cfg,
    }
  }
}
