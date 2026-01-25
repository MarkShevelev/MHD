pub fn transpose_blocked_f32(dst: &mut [f32], src: &[f32], rows: usize, cols: usize) {
  let block_size = 32;
  for i in (0..rows).step_by(block_size) {
    for j in (0..cols).step_by(block_size) {
      // Process the B x B block
      for ii in i..i + block_size {
        for jj in j..j + block_size {
          dst[jj * rows + ii] = src[ii * cols + jj];
        }
      }
    }
  }
}

use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
unsafe fn transpose_8x8_f32_avx(src: *const f32, dst: *mut f32, lda: usize, ldb: usize) {
  unsafe {
    // 1. Load 8 rows (each 8 floats = 256 bits)
    let r0 = _mm256_loadu_ps(src.add(0 * lda));
    let r1 = _mm256_loadu_ps(src.add(1 * lda));
    let r2 = _mm256_loadu_ps(src.add(2 * lda));
    let r3 = _mm256_loadu_ps(src.add(3 * lda));
    let r4 = _mm256_loadu_ps(src.add(4 * lda));
    let r5 = _mm256_loadu_ps(src.add(5 * lda));
    let r6 = _mm256_loadu_ps(src.add(6 * lda));
    let r7 = _mm256_loadu_ps(src.add(7 * lda));

    // 2. Interleave rows (Unpack 32-bit elements)
    // Creates pairs: [A0, B0, A1, B1,  A4, B4, A5, B5]
    let t0 = _mm256_unpacklo_ps(r0, r1);
    let t1 = _mm256_unpackhi_ps(r0, r1);
    let t2 = _mm256_unpacklo_ps(r2, r3);
    let t3 = _mm256_unpackhi_ps(r2, r3);
    let t4 = _mm256_unpacklo_ps(r4, r5);
    let t5 = _mm256_unpackhi_ps(r4, r5);
    let t6 = _mm256_unpacklo_ps(r6, r7);
    let t7 = _mm256_unpackhi_ps(r6, r7);

    // 3. Interleave 64-bit blocks (using shuffle)
    // Creates quads: [A0, B0, C0, D0,  A4, B4, C4, D4]
    let q0 = _mm256_shuffle_ps(t0, t2, 0x44); 
    let q1 = _mm256_shuffle_ps(t0, t2, 0xEE);
    let q2 = _mm256_shuffle_ps(t1, t3, 0x44);
    let q3 = _mm256_shuffle_ps(t1, t3, 0xEE);
    let q4 = _mm256_shuffle_ps(t4, t6, 0x44);
    let q5 = _mm256_shuffle_ps(t4, t6, 0xEE);
    let q6 = _mm256_shuffle_ps(t5, t7, 0x44);
    let q7 = _mm256_shuffle_ps(t5, t7, 0xEE);

    // 4. Final Lane Swap (Permute 128-bit blocks)
    // This moves the [A4..D4] blocks into their correct final rows
    _mm256_storeu_ps(dst.add(0 * ldb), _mm256_permute2f128_ps(q0, q4, 0x20));
    _mm256_storeu_ps(dst.add(1 * ldb), _mm256_permute2f128_ps(q1, q5, 0x20));
    _mm256_storeu_ps(dst.add(2 * ldb), _mm256_permute2f128_ps(q2, q6, 0x20));
    _mm256_storeu_ps(dst.add(3 * ldb), _mm256_permute2f128_ps(q3, q7, 0x20));
    _mm256_storeu_ps(dst.add(4 * ldb), _mm256_permute2f128_ps(q0, q4, 0x31));
    _mm256_storeu_ps(dst.add(5 * ldb), _mm256_permute2f128_ps(q1, q5, 0x31));
    _mm256_storeu_ps(dst.add(6 * ldb), _mm256_permute2f128_ps(q2, q6, 0x31));
    _mm256_storeu_ps(dst.add(7 * ldb), _mm256_permute2f128_ps(q3, q7, 0x31));
  }
}

pub fn transpose_blocked_simd_f32(dst: &mut [f32], src: &[f32], rows: usize, cols: usize) {
  let b = 8; // Block size matches our SIMD kernel
  for i in (0..rows).step_by(b) {
    for j in (0..cols).step_by(b) {
      unsafe {
        // call SIMD kernel
        transpose_8x8_f32_avx(
          src[i * cols + j..].as_ptr(),
          dst[j * rows + i..].as_mut_ptr(),
          cols,
          rows,
        );
      }
    }
  }
}

use crate::mesh_st::{U2dViewf32};

pub fn debug_print_2d(u: U2dViewf32)
{
  println!("{:>8} {:>8} {:>8} {:>8} {:>8}","row", "col", "rho", "mnt", "bz");
  for r in u.cfg.offset_rows..(u.cfg.offset_rows + u.cfg.process_rows)
  {
    for c in u.cfg.offset_cols..(u.cfg.offset_cols + u.cfg.process_cols)
    {
      println!("{:>8} {:>8} {:>8.5} {:>8.5} {:>8.5}",
        r - u.cfg.offset_rows, c - u.cfg.offset_cols,
        u.rho[r * u.cfg.total_cols + c],
        u.mnt[r * u.cfg.total_cols + c],
        u.bz [r * u.cfg.total_cols + c]);
    }
  }
}
