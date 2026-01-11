#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

const MESH_SIZE: usize  = 1024usize;

mod mesh_st;
mod mesh_fn;
mod central_flux;
mod lxf_flux;
mod rusanov_flux;
mod roe_flux;

fn rho_init_pulse (
  rho: &mut [f32], flat: f32, bump: f32, center: usize, spread: usize)
{
  for el in rho.iter_mut()
  {
    *el = flat;
  }

  let beg = center - spread / 2;
  let end = center + spread / 2;

  for i in beg..end
  {
    rho[i] = bump;
  }
}

fn mnt_init_zero ( mnt: &mut [f32])
{
  for el in mnt.iter_mut()
  {
    *el = 0.0f32;
  }
}

struct UVecf32
{
  rho: Vec<f32>,
  mnt: Vec<f32>,
  bz : Vec<f32>,
}

impl UVecf32
{
  pub fn new (mesh_size: usize) -> Self
  {
    Self
    {
      rho: vec![0.0f32; mesh_size],
      mnt: vec![0.0f32; mesh_size],
      bz : vec![0.0f32; mesh_size],
    }
  }
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "CLA --c0 0.5 --dt 0.5 --dx 1.0 --iter 1")]
struct Args {
    /// Must be > 0 (default: 1.0)
    #[arg(long, default_value_t = 0.5, value_parser = validate_gt_zero_f32)]
    c0: f32,

    /// Must be > 0 (default: 0.5)
    #[arg(long, default_value_t = 0.5, value_parser = validate_gt_zero_f32)]
    dt: f32,

    /// Must be > 0 (default: 1.0)
    #[arg(long, default_value_t = 1.0, value_parser = validate_gt_zero_f32)]
    dx: f32,

    /// Must be > 0 (default: 1)
    #[arg(long, default_value_t = 1, value_parser = validate_gt_zero_u32)]
    iter: u32,
}

// Validation for f32 parameters
fn validate_gt_zero_f32(s: &str) -> Result<f32, String> {
    let val: f32 = s.parse().map_err(|_| format!("`{}` is not a valid number", s))?;
    if val > 0.0 {
        Ok(val)
    } else {
        Err(format!("Value must be greater than 0, found {}", val))
    }
}

// Validation for u32 parameters
fn validate_gt_zero_u32(s: &str) -> Result<u32, String> {
    let val: u32 = s.parse().map_err(|_| format!("`{}` is not a valid integer", s))?;
    if val > 0 {
        Ok(val)
    } else {
        Err(format!("Repeat must be at least 1, found {}", val))
    }
}

fn args_debug_print(args: &Args)
{
  println!("{:>8} {:>8} {:>8} {:>8}","iter", "c0", "dt", "dx");
  println!("{:8} {:>8.5} {:>8.5} {:>8.5}", args.iter, args.c0, args.dt, args.dx);
}

pub fn main() {
  let args = Args::parse();

  let mut uvec_curr = UVecf32::new(MESH_SIZE + 2);
  let mut uvec_next = UVecf32::new(MESH_SIZE + 2);
  let mut fvec_curr = UVecf32::new(MESH_SIZE + 1);

  let c0   = args.c0;
  let dt   = args.dt;
  let dx   = args.dx;
  let iter = args.iter;
  let mut u_curr = mesh_st::Uf32::new(&mut uvec_curr.rho, &mut uvec_curr.mnt, &mut uvec_curr.bz);
  let mut u_next = mesh_st::Uf32::new(&mut uvec_next.rho, &mut uvec_next.mnt, &mut uvec_next.bz);
  let mut f_curr = mesh_st::Uf32::new(&mut fvec_curr.rho, &mut fvec_curr.mnt, &mut fvec_curr.bz);

  rho_init_pulse(&mut u_curr.rho, 1.0f32, 2.0f32, MESH_SIZE >> 1, MESH_SIZE >> 4);
  // mnt zero
  // bz  zero
  // fluxes are zero

  { // cacl_flux -> apply_flux -> calc_bu -> swap
    // main loop
    for _ in 0..iter
    {
      // lxf_flux::lxf_flux_f32(&mut f_curr, &u_curr, c0, dt/dx);
      // rusanov_flux::rusanov_flux_f32(&mut f_curr, &u_curr, c0, dt/dx);
      roe_flux::roe_flux_f32(&mut f_curr, &u_curr, c0, dt/dx);
      mesh_fn::u_advance_f32(&mut u_next, &u_curr, &f_curr, dt/dx);

      u_next.rho[0] = u_next.rho[1];
      u_next.rho[u_next.rho.len() - 1] = u_next.rho[u_next.rho.len() - 2];

      u_next.mnt[0] = u_next.mnt[1];
      u_next.mnt[u_next.mnt.len() - 1] = u_next.mnt[u_next.mnt.len() - 2];

      u_next.bz[0] = u_next.bz[1];
      u_next.bz[u_next.bz.len() - 1] = u_next.bz[u_next.bz.len() - 2];

      (u_curr, u_next ) = (u_next , u_curr);
    }
  }

  args_debug_print(&args);
  mesh_fn::debug_print(&u_curr);
}
