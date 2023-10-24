pub struct IdGen;

static mut ID: u32 = 0;
static mut INIT: bool = true;

impl IdGen {
    pub fn gen_string() -> String {
        unsafe {
            if !INIT {
                ID += 1;
            } else {
                INIT = false
            }

            format!(":n{ID}")
        }
    }

    pub fn gen_u32() -> u32 {
        unsafe {
            if !INIT {
                ID += 1;
            } else {
                INIT = false
            }

            ID
        }
    }
}
