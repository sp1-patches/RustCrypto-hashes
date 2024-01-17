extern "C" {
    fn syscall_sha256_extend(w: *mut u32);
    fn syscall_sha256_compress(w: *mut u32, state: *mut u32);
}

#[inline]
pub fn compress(state: &mut [u32; 8], blocks: &[[u8; 64]]) {
    unsafe {
        for i in 0..blocks.len() {
            let mut w = [0u32; 64];
            // for j in 0..16 {
            //     w[j] = u32::from_be_bytes([
            //         blocks[i][j * 4],
            //         blocks[i][j * 4 + 1],
            //         blocks[i][j * 4 + 2],
            //         blocks[i][j * 4 + 3],
            //     ]);
            // }
            // let mut chunk: [u32; 16] = unsafe { std::mem::transmute(blocks[i]) };
            // w[0..16].copy_from_slice(&chunk);
            let mut state = [0u32; 8];
            syscall_sha256_extend(w.as_mut_ptr());
            syscall_sha256_compress(w.as_mut_ptr(), state.as_mut_ptr());
        }
    }
}
