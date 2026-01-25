use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "CLA --c0 0.5 --dt 0.5 --dx 1.0 --iter 1")]
pub struct Args {
  /// Must be > 0 (default: 1.0)
  #[arg(long, default_value_t = 0.5, value_parser = validate_gt_zero_f32)]
  pub c0: f32,

  /// Must be > 0 (default: 0.5)
  #[arg(long, default_value_t = 0.5, value_parser = validate_gt_zero_f32)]
  pub dt: f32,

  /// Must be > 0 (default: 1.0)
  #[arg(long, default_value_t = 1.0, value_parser = validate_gt_zero_f32)]
  pub dx: f32,

  /// Must be > 0 (default: 1)
  #[arg(long, default_value_t = 1, value_parser = validate_gt_zero_u32)]
  pub iter: u32,
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

pub fn args_debug_print(args: &Args)
{
  println!("{:>8} {:>8} {:>8} {:>8}","iter", "c0", "dt", "dx");
  println!("{:8} {:>8.5} {:>8.5} {:>8.5}", args.iter, args.c0, args.dt, args.dx);
}
