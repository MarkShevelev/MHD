pub struct Uf32<'a>
{
  pub rho: &'a mut [f32],
  pub mnt: &'a mut [f32],
  pub bz : &'a mut [f32],
}

impl<'a> Uf32<'a>
{
  pub fn new (
    rho: &'a mut [f32], mnt: &'a mut [f32], bz: &'a mut [f32]) -> Self
  {
    Self { rho, mnt, bz }
  }
}
