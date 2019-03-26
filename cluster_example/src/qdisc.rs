include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct Qdisc {
    nic: String,
    maj: u32,
    min: u32
}

// Bytes in second
unsafe fn call_set_rate(rate :u32, bucket :u32) -> i32 {
    // TODO: This may result in overflow.
    let rate = rate / 8 * 1000;
    set_rate ( rate as i32,
               bucket as i32)
}

impl Qdisc {
    pub fn get(nic_name: String, (tc_maj, tc_min): (u32, u32)) -> Self {
        Qdisc {
            nic: nic_name,
            maj: tc_maj,
            min: tc_min
        }
    }


    pub fn set_rate(&self, rate: u32, burst: u32) -> Result<(), ()> {
        unsafe {
            let ret = call_set_rate(rate, burst);
            if ret < 0 {
                return Err(())
            }
            Ok(())
        }
    }
}
