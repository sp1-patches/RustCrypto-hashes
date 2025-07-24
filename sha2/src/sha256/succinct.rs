extern "C" {
    fn syscall_sha256_extend(w: *mut [u64; 64]);
    fn syscall_sha256_compress(w: *mut [u64; 64], state: *mut [u64; 8]);
}

#[inline(always)]
pub fn compress(state: &mut [u32; 8], blocks: &[[u8; 64]]) {
    unsafe {
        #[repr(align(64))]
        struct W([u64; 64]);
        
        let mut w = W([0u64; 64]);
        let mut state_u64 = [0u64; 8];

        if blocks.is_empty() {
            return;
        }
        
        let first_block_aligned = blocks[0].as_ptr() as usize & 7 == 0;
        
        if first_block_aligned {
            for block in blocks {
                let src = block.as_ptr() as *const u64;
                for i in 0..8 {
                    let val = *src.add(i);
                    let b0 = (val >> 0) & 0xFF;
                    let b1 = (val >> 8) & 0xFF;
                    let b2 = (val >> 16) & 0xFF;
                    let b3 = (val >> 24) & 0xFF;
                    let b4 = (val >> 32) & 0xFF;
                    let b5 = (val >> 40) & 0xFF;
                    let b6 = (val >> 48) & 0xFF;
                    let b7 = (val >> 56) & 0xFF;
                    w.0[i*2] = ((b0 << 24) + (b1 << 16) + (b2 << 8) + b3) as u64;
                    w.0[i*2 + 1] = ((b4 << 24) + (b5 << 16) + (b6 << 8) + b7) as u64;
                }
    
                state_u64[0] = state[0] as u64;
                state_u64[1] = state[1] as u64;
                state_u64[2] = state[2] as u64;
                state_u64[3] = state[3] as u64;
                state_u64[4] = state[4] as u64;
                state_u64[5] = state[5] as u64;
                state_u64[6] = state[6] as u64;
                state_u64[7] = state[7] as u64;
                
                syscall_sha256_extend(&mut w.0);
                syscall_sha256_compress(&mut w.0, &mut state_u64);
                
                state[0] = state_u64[0] as u32;
                state[1] = state_u64[1] as u32;
                state[2] = state_u64[2] as u32;
                state[3] = state_u64[3] as u32;
                state[4] = state_u64[4] as u32;
                state[5] = state_u64[5] as u32;
                state[6] = state_u64[6] as u32;
                state[7] = state_u64[7] as u32;
            }
        } 
        else {
            for block in blocks {
                let src = block.as_ptr();
                w.0[0] = u32::from_be_bytes([*src.add(0), *src.add(1), *src.add(2), *src.add(3)]) as u64;
                w.0[1] = u32::from_be_bytes([*src.add(4), *src.add(5), *src.add(6), *src.add(7)]) as u64;
                w.0[2] = u32::from_be_bytes([*src.add(8), *src.add(9), *src.add(10), *src.add(11)]) as u64;
                w.0[3] = u32::from_be_bytes([*src.add(12), *src.add(13), *src.add(14), *src.add(15)]) as u64;
                w.0[4] = u32::from_be_bytes([*src.add(16), *src.add(17), *src.add(18), *src.add(19)]) as u64;
                w.0[5] = u32::from_be_bytes([*src.add(20), *src.add(21), *src.add(22), *src.add(23)]) as u64;
                w.0[6] = u32::from_be_bytes([*src.add(24), *src.add(25), *src.add(26), *src.add(27)]) as u64;
                w.0[7] = u32::from_be_bytes([*src.add(28), *src.add(29), *src.add(30), *src.add(31)]) as u64;
                w.0[8] = u32::from_be_bytes([*src.add(32), *src.add(33), *src.add(34), *src.add(35)]) as u64;
                w.0[9] = u32::from_be_bytes([*src.add(36), *src.add(37), *src.add(38), *src.add(39)]) as u64;
                w.0[10] = u32::from_be_bytes([*src.add(40), *src.add(41), *src.add(42), *src.add(43)]) as u64;
                w.0[11] = u32::from_be_bytes([*src.add(44), *src.add(45), *src.add(46), *src.add(47)]) as u64;
                w.0[12] = u32::from_be_bytes([*src.add(48), *src.add(49), *src.add(50), *src.add(51)]) as u64;
                w.0[13] = u32::from_be_bytes([*src.add(52), *src.add(53), *src.add(54), *src.add(55)]) as u64;
                w.0[14] = u32::from_be_bytes([*src.add(56), *src.add(57), *src.add(58), *src.add(59)]) as u64;
                w.0[15] = u32::from_be_bytes([*src.add(60), *src.add(61), *src.add(62), *src.add(63)]) as u64;
            
                state_u64[0] = state[0] as u64;
                state_u64[1] = state[1] as u64;
                state_u64[2] = state[2] as u64;
                state_u64[3] = state[3] as u64;
                state_u64[4] = state[4] as u64;
                state_u64[5] = state[5] as u64;
                state_u64[6] = state[6] as u64;
                state_u64[7] = state[7] as u64;
                
                syscall_sha256_extend(&mut w.0);
                syscall_sha256_compress(&mut w.0, &mut state_u64);
                
                state[0] = state_u64[0] as u32;
                state[1] = state_u64[1] as u32;
                state[2] = state_u64[2] as u32;
                state[3] = state_u64[3] as u32;
                state[4] = state_u64[4] as u32;
                state[5] = state_u64[5] as u32;
                state[6] = state_u64[6] as u32;
                state[7] = state_u64[7] as u32;
            }
        }
    }
}
