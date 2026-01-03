
pub fn u_advance_f32(next: &mut [f32], curr: &[f32], flow: &[f32], d: f32)
{
  for ((c, n), f) in 
    curr.iter().skip(1).zip(next.iter_mut().skip(1)).zip(flow.iter())
  {
    *n = c - d * (*f);
  }
}

pub fn f_diff_f32(f: &mut [f32])
{
  let size = f.len() - 1;
  for i in 0..size
  {
    f[i] = f[i + 1] - f[i];
  }
}
