use super::mesh_st::{Uf32};

pub fn centarl_flux_f32 (f: &mut Uf32, u: &Uf32, c0: f32, d: f32)
{
  let inputs = u.rho.windows(2)
    .zip(u.mnt.windows(2))
    .zip(u.bz.windows(2));

  let outputs = f.rho.iter_mut()
    .zip(f.mnt.iter_mut())
    .zip(f.bz.iter_mut());

  for (((rho, mnt), bz), ((f_rho, f_mnt), f_bz) ) in inputs.zip(outputs)
  {
    let left_rho_f  = mnt[0];
    let right_rho_f = mnt[1];
    *f_rho = (left_rho_f + right_rho_f) / 2.0f32;

    let f_mnt_left  = 
      rho[0] * c0 * c0 + 
      bz[0] * bz[0] * 0.5 + 
      mnt[0] * mnt[0] / rho[0];
    let f_mnt_right = 
      rho[1] * c0 * c0 + 
      bz[1] * bz[1] * 0.5 + 
      mnt[1] * mnt[1] / rho[1];
    *f_mnt = (f_mnt_left + f_mnt_right) / 2.0f32;

    let f_bz_left  = bz[0] * mnt[0] / rho[0];
    let f_bz_right = bz[1] * mnt[1] / rho[1];
    *f_bz = (f_bz_left + f_bz_right) / 2.0f32;
  }
}
