extern "C" {
    fn syscall_sha256_extend(w: *mut [u64; 64]);
    fn syscall_sha256_compress(w: *mut [u64; 64], state: *mut [u64; 8]);
}

#[inline]
pub fn compress(state: &mut [u32; 8], blocks: &[[u8; 64]]) {
    use core::convert::TryInto;
    unsafe {
        for block in blocks {
            let mut w = [0u64; 64];
            for (o, chunk) in w.iter_mut().zip(block.chunks_exact(4)) {
                *o = u32::from_be_bytes(chunk.try_into().unwrap()) as u64;
            }
            let mut state_u64 = state.map(|x| x as u64);
            syscall_sha256_extend(&mut w);
            syscall_sha256_compress(&mut w, &mut state_u64);
            for (l, r) in core::iter::zip(&mut *state, state_u64) {
                *l = r as u32;
            }
        }
    }
}
