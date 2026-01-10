use super::mesh_st::{Uf32};

pub fn rho_central_flux_f32 (f: &mut [f32], u: &Uf32, c0: f32, d: f32)
{
  for (w, f_val) in u.mnt.windows(2).zip(f.iter_mut())
  {
    let left_f  = w[0];
    let right_f = w[1];
    *f_val = (left_f + right_f) / 2.0f32;
  }
}

pub fn mnt_central_flux_f32 (f: &mut [f32], u: &Uf32, c0: f32, d: f32)
{
  let wins_rho = u.rho.windows(2);
  let wins_bz  = u.bz.windows(2);
  let wins_mnt = u.mnt.windows(2);

  for (((w_rho, w_bz), w_mnt), f_val) in 
    wins_rho.zip(wins_bz).zip(wins_mnt).zip(f.iter_mut())
  {
    let left_f  = w_rho[0] * c0 * c0 + w_bz[0] * w_bz[0] * 0.5 + w_mnt[0] * w_mnt[0] / w_rho[0];
    let right_f = w_rho[1] * c0 * c0 + w_bz[1] * w_bz[1] * 0.5 + w_mnt[1] * w_mnt[1] / w_rho[1];
    
    *f_val = (left_f + right_f) / 2.0f32;
  }
}

pub fn bz_central_flux_f32 (f: &mut [f32], u: &Uf32, c0: f32, d: f32)
{
  let rho_wins = u.rho.windows(2);
  let mnt_wins = u.mnt.windows(2);
  let bz_wins  = u.bz.windows(2);

  for (((rho_win, bz_win), mnt_win), f_val) in 
    rho_wins.zip(bz_wins).zip(mnt_wins).zip(f.iter_mut())
  {
    let left_f  = bz_win[0] * mnt_win[0] / rho_win[0];
    let right_f = bz_win[1] * mnt_win[1] / rho_win[1];
    *f_val = (left_f + right_f) / 2.0f32;
  }
}
