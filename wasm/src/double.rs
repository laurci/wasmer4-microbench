extern "C" {
    fn double(n: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn bench_double(iter: i32) {
    for i in 0..iter {
        unsafe {
            double(i);
        };
    }
}
