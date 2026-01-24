#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

const BLOCK_SIZE: usize = 32usize;
const ROW_SIZE: usize   = 32usize;
const COL_SIZE: usize   = 64usize;

mod mesh_st;
mod mesh_fn;
mod mesh_2d_fn;
mod central_flux;
mod lxf_flux;
mod rusanov_flux;
mod roe_flux;

use mesh_st::{Uf32, Uf32View};

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

  let c0   = args.c0;
  let dt   = args.dt;
  let dx   = args.dx;
  let iter = args.iter;

  let mut uvec_curr_2d = UVecf32::new(BLOCK_SIZE * BLOCK_SIZE * (ROW_SIZE + 2) * (COL_SIZE + 2));
  let mut uvec_next_2d = UVecf32::new(BLOCK_SIZE * BLOCK_SIZE * (ROW_SIZE + 2) * (COL_SIZE + 2));
  let mut fvec_2d      = UVecf32::new(BLOCK_SIZE * BLOCK_SIZE * (ROW_SIZE + 1) * (COL_SIZE + 1));

  let mut u_curr = Uf32 { 
    rho: &mut uvec_curr_2d.rho,
    mnt: &mut uvec_curr_2d.mnt,
    bz:  &mut uvec_curr_2d.bz };

  let mut u_next = Uf32 { 
    rho: &mut uvec_next_2d.rho,
    mnt: &mut uvec_next_2d.mnt,
    bz:  &mut uvec_next_2d.bz };

  let f = Uf32 { 
    rho: &mut fvec_2d.rho,
    mnt: &mut fvec_2d.mnt,
    bz:  &mut fvec_2d.bz };

  let ustride = BLOCK_SIZE * (COL_SIZE + 2);
  let urow_count = ROW_SIZE * BLOCK_SIZE;
  let ustart = BLOCK_SIZE - 1;
  let uend = BLOCK_SIZE * (COL_SIZE + 1) + 1;

  let fstride = BLOCK_SIZE * (COL_SIZE + 1);
  let frow_count = ROW_SIZE * BLOCK_SIZE;
  let fstart = 0usize;
  let fend = BLOCK_SIZE * COL_SIZE + 1;

  rho_init_pulse(&mut u_curr.rho, 1.0f32, 2.0f32, BLOCK_SIZE * COL_SIZE >> 1, BLOCK_SIZE * COL_SIZE >> 4);

  { // cacl_flux -> apply_flux -> calc_bu -> swap
    // main loop
    for _ in 0..iter
    {
      let u_curr_rho_rows = u_curr.rho.chunks_exact_mut(ustride);
      let u_curr_mnt_rows = u_curr.mnt.chunks_exact_mut(ustride);
      let u_curr_bz_rows  = u_curr.bz.chunks_exact_mut( ustride);

      let f_rho_rows = f.rho.chunks_exact_mut(fstride);
      let f_mnt_rows = f.mnt.chunks_exact_mut(fstride);
      let f_bz_rows  = f.bz.chunks_exact_mut( fstride);

      let u_next_rho_rows = u_next.rho.chunks_exact_mut(ustride);
      let u_next_mnt_rows = u_next.mnt.chunks_exact_mut(ustride);
      let u_next_bz_rows  = u_next.bz.chunks_exact_mut( ustride);

      let zipped = 
        u_curr_rho_rows.zip(u_curr_mnt_rows).zip(u_curr_bz_rows)
        .zip(f_rho_rows).zip(f_mnt_rows).zip(f_bz_rows)
        .zip(u_next_rho_rows).zip(u_next_mnt_rows).zip(u_next_bz_rows);

      for ((((((((uc_rho, uc_mnt), uc_bz), f_rho), f_mnt), f_bz), un_rho), un_mnt), un_bz) in zipped
      {
        {
          let u_curr_1d     = Uf32View::new_with_slice(
            uc_rho, uc_mnt, uc_bz, ustart, uend);
          let f_1d      = Uf32::new_with_slice(
            f_rho, f_mnt, f_bz, fstart, fend);

          // lxf_flux::lxf_flux_f32(&mut f_curr, &u_curr, c0, dt/dx);
          // rusanov_flux::rusanov_flux_f32(&mut f_curr, &u_curr, c0, dt/dx);
          roe_flux::roe_flux_f32(f_1d, u_curr_1d, c0, dt/dx);
        }

        {
          let u_curr_1d = Uf32View::new_with_slice(
            uc_rho, uc_mnt, uc_bz, ustart, uend);
          let u_next_1d = Uf32::new_with_slice(
            un_rho, un_mnt, un_bz, ustart, uend);
          let f_1d = Uf32View::new_with_slice(
            f_rho, f_mnt, f_bz, fstart, fend);

          mesh_fn::u_advance_f32(u_next_1d, u_curr_1d, f_1d, dt/dx);
        }

        {
          let u_next_1d = Uf32::new_with_slice(
            un_rho, un_mnt, un_bz, ustart, uend);

          u_next_1d.rho[0] = u_next_1d.rho[1];
          u_next_1d.rho[u_next_1d.rho.len() - 1] = u_next_1d.rho[u_next_1d.rho.len() - 2];

          u_next_1d.mnt[0] = u_next_1d.mnt[1];
          u_next_1d.mnt[u_next_1d.mnt.len() - 1] = u_next_1d.mnt[u_next_1d.mnt.len() - 2];

          u_next_1d.bz[0] = u_next_1d.bz[1];
          u_next_1d.bz[u_next_1d.bz.len() - 1] = u_next_1d.bz[u_next_1d.bz.len() - 2];
        }
      }

      (u_curr, u_next ) = (u_next , u_curr);
    }
  }

  args_debug_print(&args);
  mesh_2d_fn::debug_print_2d(Uf32View::from(&u_curr), BLOCK_SIZE * ROW_SIZE, BLOCK_SIZE * COL_SIZE);
}
