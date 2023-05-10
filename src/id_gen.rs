pub struct IdGen;

static mut ID: u32 = 0;

impl IdGen {

    pub fn get_and_update() -> String {

        unsafe {
            ID += 1;
        }

        return unsafe {
            format!(":n{ID}")
        }
    }
}
