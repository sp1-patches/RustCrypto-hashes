extern "C" {
    fn syscall_keccak_permute(state: *mut [u64; 25]);
}

#[inline]
pub(crate) fn keccak_permute(state: &mut [u64; 25]) {
    unsafe {
        syscall_keccak_permute(state);
    }
}
