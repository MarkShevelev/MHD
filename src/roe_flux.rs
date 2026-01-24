use crate::mesh_st::{Uf32, Uf32View};

pub fn roe_flux_f32 (f: Uf32, u: Uf32View, c0: f32, d: f32)
{
  let inputs = u.rho.windows(2)
    .zip(u.mnt.windows(2))
    .zip(u.bz.windows(2));

  let outputs = f.rho.iter_mut()
    .zip(f.mnt.iter_mut())
    .zip(f.bz.iter_mut());

  for (((rho, mnt), bz), ((f_rho, f_mnt), f_bz) ) in inputs.zip(outputs)
  {
    let rhol = rho[0]; 
    let rhor = rho[1]; 
    let ul   = mnt[0]/rhol;
    let ur   = mnt[1]/rhor;
    let bzl  = bz[0];
    let bzr  = bz[1];
    
    let sqrt_rhol = rhol.sqrt();
    let sqrt_rhor = rhor.sqrt();
    let sqrt_rho_sum = sqrt_rhol + sqrt_rhor;
    let u_tilde = (sqrt_rhol * ul + sqrt_rhor * ur) / sqrt_rho_sum;
    let bz_tilde = (sqrt_rhol * bzl + sqrt_rhor * bzr) / sqrt_rho_sum;
    let rho_tilde = (rhol * rhor).sqrt();
    let cf_tilde = (c0 * c0 + bz_tilde * bz_tilde / rho_tilde).sqrt();
    
    let lambda1 = u_tilde - c0;
    let lambda2 = u_tilde;
    let lambda3 = u_tilde + c0;
    
    let drho = rho[1] - rho[0];
    let dmnt = mnt[1] - mnt[0];
    let dbz  = bz[1] - bz[0];
    
    let bz_corr = 0.5 * (
      if bz_tilde.abs() > 1.0e-9 { rho_tilde * dbz / bz_tilde } 
      else { drho }
    );
    let alpha1 = 0.5 * ((u_tilde + c0)*drho - dmnt) / cf_tilde + bz_corr;
    let alpha3 = 0.5 * (dmnt - (u_tilde - c0)*drho) / cf_tilde + bz_corr;
    let alpha2 = 
      if bz_tilde.abs() > 1.0e-9 { drho - alpha1 - alpha3 } else { 0.0 };

    let f_rho_left  = mnt[0];
    let f_rho_right = mnt[1];
    
    *f_rho = 0.5 * (f_rho_left + f_rho_right) 
      - 0.5*(lambda1.abs() * alpha1 + 
             lambda2.abs() * alpha2 + 
             lambda3.abs() * alpha3);
    
    let f_mnt_left  = rho[0] * c0 * c0 + bz[0] * bz[0] * 0.5 + mnt[0] * mnt[0] / rho[0];
    let f_mnt_right = rho[1] * c0 * c0 + bz[1] * bz[1] * 0.5 + mnt[1] * mnt[1] / rho[1];
    *f_mnt = 0.5 * (f_mnt_left + f_mnt_right) 
      - 0.5*(lambda1.abs() * alpha1 * (u_tilde - cf_tilde) + 
             lambda2.abs() * alpha2 * u_tilde + 
             lambda3.abs() * alpha3 * (u_tilde + cf_tilde));

    let f_bz_left  = bz[0] * mnt[0] / rho[0];
    let f_bz_right = bz[1] * mnt[1] / rho[1];
    
    *f_bz = 0.5 * (f_bz_left + f_bz_right) 
      - 0.5*(lambda1.abs() * alpha1 * bz_tilde / rho_tilde + 
             lambda3.abs() * alpha3 * bz_tilde / rho_tilde);
  }
}
