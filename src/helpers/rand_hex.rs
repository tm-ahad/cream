use std::arch::x86_64::_rdrand32_step;

pub fn rand_hex() -> String {
    let mut rand_n: u32 = 0;

    unsafe {
        return if _rdrand32_step(&mut rand_n) == 1 {
            format!("f{:x}", rand_n)
        } else {
            rand_hex()
        }
    }
}
