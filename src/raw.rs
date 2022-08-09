use libc::{c_double, c_int, c_void};

#[link(name = "rbiteoptglue", kind = "static")]
extern "C" {
    pub(crate) fn biteopt_optimize(
        minimized_out: *mut c_double,
        dimension: c_int,
        userdata: *const c_void,
        lower_bound: c_double,
        upper_bound: c_double,
        iter: c_int,
        depth: c_int,
        attc: c_int,
        evaluate: extern "C" fn(c_int, *const c_double, *const c_void) -> c_double,
    );
}
