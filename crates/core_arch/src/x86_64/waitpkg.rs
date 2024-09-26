//! x86_64 waitpkg intrinsics.

#[cfg(test)]
use stdarch_test::assert_instr;

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.x86.tpause"]
    fn tpause(ctrl: u32, counter_hi: u32, counter_lo: u32) -> u8;
    #[link_name = "llvm.x86.umonitor"]
    fn umonitor(a: *const ());
    #[link_name = "llvm.x86.umwait"]
    fn umwait(ctrl: u32, counter_hi: u32, counter_lo: u32) -> u8;
}

/// Sets up a linear address range to be monitored by hardware and 
/// activates the monitor. 
/// 
/// The address range should be a writeback memory caching type. 
/// 
/// The address is contained in `a`.
/// 
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=umonitor)
#[inline]
#[target_feature(enable = "waitpkg")]
#[cfg_attr(test, assert_instr(umonitor))]
#[unstable(feature = "x86_waitpkg_intrinsics", issue = "")]
pub unsafe fn _umonitor<T>(a: *const T) {
    umonitor(a as *const ())
}

/// Directs the processor to enter an implementation-dependent optimized
/// state while monitoring a range of addresses. 
/// 
/// The instruction wakes up when the TSC reaches or exceeds the value 
/// specified in counter (if the monitoring hardware did not trigger 
/// beforehand). 
/// 
/// Bit 0 of `ctrl` selects between a lower power (cleared) or faster wakeup
/// (set) optimized state. 
/// 
/// Returns the carry flag (CF). 
/// 
/// If the processor that executed a UMWAIT instruction wakes due to the
/// expiration of the operating system timelimit, the instructions sets 
/// RFLAGS.CF; otherwise, that flag is cleared.
/// 
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=umwait)
#[inline]
#[target_feature(enable = "waitpkg")]
#[cfg_attr(test, assert_instr(umwait))]
#[unstable(feature = "x86_waitpkg_intrinsics", issue = "")]
pub unsafe fn _umwait(ctrl: u32, counter: u64) -> u8 {
    umwait(ctrl, (counter >> 32) as u32, counter as u32)
}

/// Directs the processor to enter an implementation-dependent optimized 
/// state until the TSC reaches or exceeds the value specified in `counter`. 
/// 
/// Bit 0 of `ctrl` selects between a lower power (cleared) or faster wakeup 
/// (set) optimized state.
/// 
/// Returns the carry flag (CF). 
/// 
/// If the processor that executed a UMWAIT instruction wakes due to the 
/// expiration of the operating system timelimit, the instructions sets 
/// RFLAGS.CF; otherwise, that flag is cleared.
/// 
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=tpause)
#[inline]
#[target_feature(enable = "waitpkg")]
#[cfg_attr(test, assert_instr(tpause))]
#[unstable(feature = "x86_waitpkg_intrinsics", issue = "")]
pub unsafe fn _tpause(ctrl: u32, counter: u64) -> u8 {
    tpause(ctrl, (counter >> 32) as u32, counter as u32)
}

#[cfg(test)]
mod test {
    use super::*;
    use stdarch_test::assert_instr;

    
}