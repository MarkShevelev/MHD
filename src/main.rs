#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

const MESH_SIZE: usize  = 1024usize;
#[allow(non_upper_case_globals)]
const dx: f32           = 1.0f32;
#[allow(non_upper_case_globals)]
const dt: f32           = 0.5f32;

mod mesh_st;
mod mesh_fn;
mod central_flux;
mod lxf_flux;

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

pub fn debug_mesh_print(u: &mesh_st::Uf32)
{
  println!("{:>5} {:>8} {:>8} {:>8}","iter", "rho", "mnt", "bz");
  let size = u.rho.len();
  for i in 0..size
  {
    println!("{:5} {:>8.5} {:>8.5} {:>8.5}", i, u.rho[i], u.mnt[i], u.bz[i]);
  }
}

pub fn main() {
  let mut uvec_curr = UVecf32::new(MESH_SIZE + 2);
  let mut uvec_next = UVecf32::new(MESH_SIZE + 2);
  let mut fvec_curr = UVecf32::new(MESH_SIZE + 1);

  let c0 = 0.5f32;
  let mut u_curr = mesh_st::Uf32::new(&mut uvec_curr.rho, &mut uvec_curr.mnt, &mut uvec_curr.bz);
  let mut u_next = mesh_st::Uf32::new(&mut uvec_next.rho, &mut uvec_next.mnt, &mut uvec_next.bz);
  let mut f_curr = mesh_st::Uf32::new(&mut fvec_curr.rho, &mut fvec_curr.mnt, &mut fvec_curr.bz);

  rho_init_pulse(&mut u_curr.rho, 1.0f32, 2.0f32, MESH_SIZE >> 1, MESH_SIZE >> 4);
  // mnt zero
  // bz  zero
  // fluxes are zero

  { // cacl_flux -> cacl_bflux -> diff_flux -> apply_flux -> calc_bu -> swap
    // main loop
    for _ in 0..1800
    {
      lxf_flux::rho_lxf_flux_f32(&mut f_curr.rho, &u_curr, dt/dx);
      lxf_flux::mnt_lxf_flux_f32(&mut f_curr.mnt, &u_curr, c0, dt/dx);
      lxf_flux::bz_lxf_flux_f32(&mut f_curr.bz, &u_curr, dt/dx);

      mesh_fn::f_diff_f32(&mut f_curr.rho);
      mesh_fn::f_diff_f32(&mut f_curr.mnt);
      mesh_fn::f_diff_f32(&mut f_curr.bz);

      mesh_fn::u_advance_f32(&mut u_next.rho, &u_curr.rho, &f_curr.rho, dt/dx);
      mesh_fn::u_advance_f32(&mut u_next.mnt, &u_curr.mnt, &f_curr.mnt, dt/dx);
      mesh_fn::u_advance_f32(&mut u_next.bz, &u_curr.bz, &f_curr.bz, dt/dx);

      u_next.rho[0] = u_next.rho[1];
      u_next.rho[u_next.rho.len() - 1] = u_next.rho[u_next.rho.len() - 2];

      u_next.mnt[0] = u_next.mnt[1];
      u_next.mnt[u_next.mnt.len() - 1] = u_next.mnt[u_next.mnt.len() - 2];

      u_next.bz[0] = u_next.bz[1];
      u_next.bz[u_next.bz.len() - 1] = u_next.bz[u_next.bz.len() - 2];

      (u_curr, u_next ) = (u_next , u_curr);
    }
  }

  debug_mesh_print(&u_curr);
}
