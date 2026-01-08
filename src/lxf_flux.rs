use super::mesh_st::{Uf32};

pub fn rho_lxf_flux_f32(f: &mut [f32], u: &Uf32, c0: f32, d: f32)
{
  for ( (w_mnt, w_rho), f_val) in u.mnt.windows(2).zip(u.rho.windows(2)).zip(f.iter_mut())
  {
    let left_f  = w_mnt[0];
    let right_f = w_mnt[1];
    *f_val = (left_f + right_f) / 2.0f32 - (w_rho[1] - w_rho[0]) / (2.0f32 * d);
  }
}

pub fn mnt_lxf_flux_f32 (f: &mut [f32], u: &Uf32, c0: f32, d: f32)
{
  let wins_rho = u.rho.windows(2);
  let wins_bz  = u.bz.windows(2);
  let wins_mnt = u.mnt.windows(2);

  for (((w_rho, w_bz), w_mnt), f_val) in 
    wins_rho.zip(wins_bz).zip(wins_mnt).zip(f.iter_mut())
  {
    let left_f  = w_rho[0] * c0 + w_bz[0] * w_bz[0] * 0.5;
    let right_f = w_rho[1] * c0 + w_bz[1] * w_bz[1] * 0.5;
    
    *f_val = (left_f + right_f) / 2.0f32 - (w_mnt[1] - w_mnt[0]) / (2.0f32 * d);
  }
}

pub fn bz_lxf_flux_f32 (f: &mut [f32], u: &Uf32, c0: f32, d: f32)
{
  let wins_rho = u.rho.windows(2);
  let wins_bz  = u.bz.windows(2);
  let wins_mnt = u.mnt.windows(2);

  for (((w_rho, w_bz), w_mnt), f_val) in 
    wins_rho.zip(wins_bz).zip(wins_mnt).zip(f.iter_mut())
  {
    let left_f  = w_bz[0] * w_mnt[0] / w_rho[0];
    let right_f = w_bz[1] * w_mnt[1] / w_rho[1];
    *f_val = (left_f + right_f) / 2.0f32 - (w_bz[1] - w_bz[0]) / (2.0f32 * d);
  }
}
