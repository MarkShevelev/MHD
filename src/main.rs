#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod cla;
mod mesh_st;
mod mesh_fn;
mod mesh_2d_fn;
mod central_flux;
mod lxf_flux;
mod rusanov_flux;
mod roe_flux;

use clap::Parser;

use mesh_st::{Uf32, Uf32View, U2dCfg, U2df32, U2dViewf32};

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

fn rho_init_pulse_2d (
  u: &mut U2df32, flat: f32, bump: f32, center: usize, spread: usize)
{
  let u_rho_rows = u.rho
    .chunks_exact_mut(u.cfg.total_cols);

  for rho in u_rho_rows
  {
    rho_init_pulse(rho, flat, bump, center, spread);
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

struct U2dVecf32
{
  rho: Vec<f32>,
  mnt: Vec<f32>,
  bz : Vec<f32>,
  cfg: U2dCfg,
}

impl U2dVecf32
{
  pub fn new (
    cfg: U2dCfg) -> Self
  {
    let mesh_size = cfg.total_rows * cfg.total_cols;
    Self {
      rho: vec![0.0f32; mesh_size],
      mnt: vec![0.0f32; mesh_size],
      bz : vec![0.0f32; mesh_size],
      cfg: cfg
    }
  }
}

const BLOCK_SIZE: usize = 32usize;
const BLOCK_ROWS: usize = 32usize;
const BLOCK_COLS: usize = 64usize;
const ROW_SIZE: usize   = 32usize;
const COL_SIZE: usize   = 64usize;

pub fn main() {
  let args = cla::Args::parse();

  let c0   = args.c0;
  let dt   = args.dt;
  let dx   = args.dx;
  let iter = args.iter;

  let ucfg = U2dCfg { 
    total_rows : BLOCK_SIZE * (BLOCK_ROWS + 2),
    total_cols : BLOCK_SIZE * (BLOCK_COLS + 2),
    offset_rows: BLOCK_SIZE,
    offset_cols: BLOCK_SIZE - 1,
    process_rows: BLOCK_SIZE * BLOCK_ROWS,
    process_cols: BLOCK_SIZE * BLOCK_COLS + 2};

  let fcfg = U2dCfg { 
    total_rows : BLOCK_SIZE * (BLOCK_ROWS + 1),
    total_cols : BLOCK_SIZE * (BLOCK_COLS + 1),
    offset_rows: BLOCK_SIZE,
    offset_cols: 0,
    process_rows: BLOCK_SIZE * BLOCK_ROWS,
    process_cols: BLOCK_SIZE * BLOCK_COLS + 1};

  let mut uvec_curr_2d = U2dVecf32::new(ucfg);
  let mut uvec_next_2d = U2dVecf32::new(ucfg);
  let mut fvec_2d      = U2dVecf32::new(fcfg);

  let mut u_curr = U2df32 { 
    rho: &mut uvec_curr_2d.rho,
    mnt: &mut uvec_curr_2d.mnt,
    bz:  &mut uvec_curr_2d.bz,
    cfg: ucfg,};
  let mut u_next = U2df32 { 
    rho: &mut uvec_next_2d.rho,
    mnt: &mut uvec_next_2d.mnt,
    bz:  &mut uvec_next_2d.bz,
    cfg: ucfg,};

  let f = U2df32 { 
    rho: &mut fvec_2d.rho,
    mnt: &mut fvec_2d.mnt,
    bz:  &mut fvec_2d.bz,
    cfg: ucfg,};


  rho_init_pulse_2d(
    &mut u_curr, 1.0f32, 2.0f32, 
    (ucfg.total_cols) >> 1,
    (ucfg.process_cols) >> 4);

  { // cacl_flux -> apply_flux -> calc_bu -> swap
    // main loop
    for _ in 0..iter
    {
      let ustart = ucfg.offset_cols;
      let uend   = ucfg.offset_cols + ucfg.process_cols;

      let fstart = fcfg.offset_cols;
      let fend   = fcfg.offset_cols + fcfg.process_cols;

      let u_curr_rho_rows = u_curr.rho
        .chunks_exact_mut(ucfg.total_cols)
        .skip(ucfg.offset_rows).take(ucfg.process_rows);

      let u_curr_mnt_rows = u_curr.mnt
        .chunks_exact_mut(ucfg.total_cols)
        .skip(ucfg.offset_rows).take(ucfg.process_rows);

      let u_curr_bz_rows  = u_curr.bz
        .chunks_exact_mut( ucfg.total_cols)
        .skip(ucfg.offset_rows).take(ucfg.process_rows);

      let f_rho_rows = f.rho
        .chunks_exact_mut(fcfg.total_cols)
        .skip(fcfg.offset_rows).take(fcfg.process_rows);

      let f_mnt_rows = f.mnt
        .chunks_exact_mut(fcfg.total_cols)
        .skip(fcfg.offset_rows).take(fcfg.process_rows);

      let f_bz_rows  = f.bz
        .chunks_exact_mut( fcfg.total_cols)
        .skip(fcfg.offset_rows).take(fcfg.process_rows);

      let u_next_rho_rows = u_next.rho
        .chunks_exact_mut(ucfg.total_cols)
        .skip(ucfg.offset_rows).take(ucfg.process_rows);

      let u_next_mnt_rows = u_next.mnt
        .chunks_exact_mut(ucfg.total_cols)
        .skip(ucfg.offset_rows).take(ucfg.process_rows);

      let u_next_bz_rows  = u_next.bz
        .chunks_exact_mut(ucfg.total_cols)
        .skip(ucfg.offset_rows).take(ucfg.process_rows);

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

  cla::args_debug_print(&args);
  mesh_2d_fn::debug_print_2d(U2dViewf32::from(&u_curr));
}
