pub struct IdGen;

static mut ID: u32 = 0;
static mut INIT: bool = true;

impl IdGen {

    pub fn get_and_update() -> String {

        unsafe {
            if !INIT {
                ID += 1;
            } else {
                INIT = false
            }

            format!(":n{ID}")
        }
    }
}
