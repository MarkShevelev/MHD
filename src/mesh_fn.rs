use super::mesh_st::{Uf32};

pub fn slice_advance_f32 (next: &mut [f32], curr: &[f32], flow: &[f32], d: f32)
{
  for ((c, f), n) in 
    curr.iter().skip(1).zip(flow.windows(2)).zip(next.iter_mut().skip(1))
  {
    *n = c - d * (f[1] -f[0]);
  }
}

pub fn u_advance_f32 (next: &mut Uf32, curr: &Uf32, flow: &Uf32, d: f32)
{
  let u_next = next.rho.iter_mut().skip(1)
    .zip(next.mnt.iter_mut().skip(1))
    .zip(next.bz.iter_mut().skip(1));

  let u_curr = curr.rho.iter().skip(1)
    .zip(curr.mnt.iter().skip(1))
    .zip(curr.bz.iter().skip(1));

  let f = flow.rho.windows(2)
    .zip(flow.mnt.windows(2))
    .zip(flow.bz.windows(2));

  for ((((rho_next, mnt_next), bz_next),
      ((rho_curr, mnt_curr), bz_curr)),
      ((rho_f, mnt_f), bz_f))
  in u_next.zip(u_curr).zip(f)
  {
    *rho_next = *rho_curr - d * (rho_f[1] - rho_f[0]);
    *mnt_next = *mnt_curr - d * (mnt_f[1] - mnt_f[0]);
    *bz_next  = *bz_curr  - d * (bz_f[1]  - bz_f[0]);
  }
}
