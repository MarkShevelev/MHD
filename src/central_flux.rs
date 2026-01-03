use super::mesh_st::{Uf32};

#[inline]
fn choice(left_f: f32, right_f: f32) -> f32
{
  (left_f + right_f) / 2.0
}

pub fn rho_central_flux_f32 (f: &mut [f32], u: &Uf32)
{
  for (w, f_val) in u.mnt.windows(2).zip(f.iter_mut())
  {
    let left_f  = w[0];
    let right_f = w[1];
    *f_val = choice (left_f, right_f);
  }
}

pub fn mnt_central_flux_f32 (f: &mut [f32], u: &Uf32, c0: f32)
{
  let rho_wins = u.rho.windows(2);
  let bz_wins  = u.bz.windows(2);

  for ((rho_win, bz_win), f_val) in rho_wins.zip(bz_wins).zip(f.iter_mut()) {
    let left_f  = rho_win[0] * c0 + bz_win[0] * bz_win[0] * 0.5;
    let right_f = rho_win[1] * c0 + bz_win[1] * bz_win[1] * 0.5;
    *f_val = choice (left_f, right_f);
  }
}

pub fn bz_central_flux_f32 (f: &mut [f32], u: &Uf32)
{
  let rho_wins = u.rho.windows(2);
  let mnt_wins = u.mnt.windows(2);
  let bz_wins  = u.bz.windows(2);

  for (((rho_win, bz_win), mnt_win), f_val) in 
    rho_wins.zip(bz_wins).zip(mnt_wins).zip(f.iter_mut())
  {
    let left_f  = bz_win[0] * mnt_win[0] / rho_win[0];
    let right_f = bz_win[1] * mnt_win[1] / rho_win[1];
    *f_val = choice (left_f, right_f);
  }
}
