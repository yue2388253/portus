include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Bytes in second
unsafe fn call_set_rate(rate :u32, bucket :u32){
    let rate = rate * 1000 / 8;
    set_rate ( rate as i32,
               bucket as i32);
}