use super::mesh_st::{Uf32};

pub fn rho_roe_flux_f32 (f: &mut [f32], u: &Uf32, c0: f32, d: f32)
{
  let rho_wins = u.rho.windows(2);
  let mnt_wins = u.mnt.windows(2);
  let bz_wins  = u.bz.windows(2);

  for (((w_rho, w_mnt), w_bz), f_val) in 
   rho_wins.zip(mnt_wins).zip(bz_wins).zip(f.iter_mut())
  {
    let rhol = w_rho[0]; 
    let rhor = w_rho[1]; 
    let ul   = w_mnt[0]/rhol;
    let ur   = w_mnt[1]/rhor;
    let bzl  = w_bz[0];
    let bzr  = w_bz[1];
    
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
    
    let drho = rhor - rhol;
    let dmnt = w_mnt[1] - w_mnt[0];
    let dbz = w_bz[1] - w_bz[0];
    
    let bz_corr = 0.5 * (
      if bz_tilde.abs() > 1.0e-9 { rho_tilde * dbz / bz_tilde } 
      else { drho }
    );
    let alpha1 = 0.5 * ((u_tilde + c0)*drho - dmnt) / cf_tilde + bz_corr;
    let alpha3 = 0.5 * (dmnt - (u_tilde - c0)*drho) / cf_tilde + bz_corr;
    let alpha2 = 
      if bz_tilde.abs() > 1.0e-9 { drho - alpha1 - alpha3 } else { 0.0 };
    
    let left_f  = w_mnt[0];
    let right_f = w_mnt[1];
    
    *f_val = 0.5 * (left_f + right_f) 
      - 0.5*(lambda1.abs() * alpha1 + 
             lambda2.abs() * alpha2 + 
             lambda3.abs() * alpha3);
  }
}

pub fn mnt_roe_flux_f32 (f: &mut [f32], u: &Uf32, c0: f32, d: f32)
{
  let rho_wins = u.rho.windows(2);
  let mnt_wins = u.mnt.windows(2);
  let bz_wins  = u.bz.windows(2);

  for (((w_rho, w_mnt), w_bz), f_val) in 
   rho_wins.zip(mnt_wins).zip(bz_wins).zip(f.iter_mut())
  {
    let rhol = w_rho[0]; 
    let rhor = w_rho[1]; 
    let ul   = w_mnt[0]/rhol;
    let ur   = w_mnt[1]/rhor;
    let bzl  = w_bz[0];
    let bzr  = w_bz[1];
    
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
    
    let drho = rhor - rhol;
    let dmnt = w_mnt[1] - w_mnt[0];
    let dbz = w_bz[1] - w_bz[0];
    
    let bz_corr = 0.5 * (
      if bz_tilde.abs() > 1.0e-9 { rho_tilde * dbz / bz_tilde } 
      else { drho }
    );
    let alpha1 = 0.5 * ((u_tilde + c0)*drho - dmnt) / cf_tilde + bz_corr;
    let alpha3 = 0.5 * (dmnt - (u_tilde - c0)*drho) / cf_tilde + bz_corr;
    let alpha2 = 
      if bz_tilde.abs() > 1.0e-9 { drho - alpha1 - alpha3 } else { 0.0 };
    
    let left_f  = w_rho[0] * c0 * c0 + w_bz[0] * w_bz[0] * 0.5 + w_mnt[0] * w_mnt[0] / w_rho[0];
    let right_f = w_rho[1] * c0 * c0 + w_bz[1] * w_bz[1] * 0.5 + w_mnt[1] * w_mnt[1] / w_rho[1];
    
    *f_val = 0.5 * (left_f + right_f) 
      - 0.5*(lambda1.abs() * alpha1 * (u_tilde - cf_tilde) + 
             lambda2.abs() * alpha2 * u_tilde + 
             lambda3.abs() * alpha3 * (u_tilde + cf_tilde));
  }
}

pub fn bz_roe_flux_f32 (f: &mut [f32], u: &Uf32, c0: f32, d: f32)
{
  let rho_wins = u.rho.windows(2);
  let mnt_wins = u.mnt.windows(2);
  let bz_wins  = u.bz.windows(2);

  for (((w_rho, w_mnt), w_bz), f_val) in 
   rho_wins.zip(mnt_wins).zip(bz_wins).zip(f.iter_mut())
  {
    let rhol = w_rho[0]; 
    let rhor = w_rho[1]; 
    let ul   = w_mnt[0]/rhol;
    let ur   = w_mnt[1]/rhor;
    let bzl  = w_bz[0];
    let bzr  = w_bz[1];
    
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
    
    let drho = rhor - rhol;
    let dmnt = w_mnt[1] - w_mnt[0];
    let dbz = w_bz[1] - w_bz[0];
    
    let bz_corr = 0.5 * (
      if bz_tilde.abs() > 1.0e-9 { rho_tilde * dbz / bz_tilde } 
      else { drho }
    );
    let alpha1 = 0.5 * ((u_tilde + c0)*drho - dmnt) / cf_tilde + bz_corr;
    let alpha3 = 0.5 * (dmnt - (u_tilde - c0)*drho) / cf_tilde + bz_corr;
    let alpha2 = 
      if bz_tilde.abs() > 1.0e-9 { drho - alpha1 - alpha3 } else { 0.0 };
    
    let left_f  = w_bz[0] * w_mnt[0] / w_rho[0];
    let right_f = w_bz[1] * w_mnt[1] / w_rho[1];
    
    *f_val = 0.5 * (left_f + right_f) 
      - 0.5*(lambda1.abs() * alpha1 * bz_tilde / rho_tilde + 
             lambda3.abs() * alpha3 * bz_tilde / rho_tilde);
  }
}
