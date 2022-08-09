mod raw;

use libc::{c_double, c_int, c_void};
use rcmaes::Vectorizable;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct BiteOptParameters {
    iter: i32,
    depth: i32,
    attc: i32,
    lower_bound: f64,
    upper_bound: f64,
}

impl BiteOptParameters {
    pub fn new() -> BiteOptParameters {
        BiteOptParameters::default()
    }

    pub fn lower_bound(&self) -> f64 {
        self.lower_bound
    }

    pub fn upper_bound(&self) -> f64 {
        self.upper_bound
    }

    pub fn set_lower_bound(&mut self, lower_bound: f64) {
        self.lower_bound = lower_bound;
    }

    pub fn set_upper_bound(&mut self, upper_bound: f64) {
        self.upper_bound = upper_bound;
    }

    pub fn iter(&self) -> i32 {
        self.iter
    }

    pub fn depth(&self) -> i32 {
        self.depth
    }

    pub fn attc(&self) -> i32 {
        self.attc
    }

    pub fn set_iter(&mut self, iter: i32) {
        self.iter = iter;
    }

    pub fn set_depth(&mut self, depth: i32) {
        self.depth = depth;
    }

    pub fn set_attc(&mut self, attc: i32) {
        self.attc = attc;
    }
}

impl Default for BiteOptParameters {
    fn default() -> BiteOptParameters {
        BiteOptParameters {
            iter: 1000,
            depth: 1,
            attc: 10,
            lower_bound: -1.0,
            upper_bound: 1.0,
        }
    }
}

struct Userdata<'a> {
    evaluate: Box<dyn Fn(&'a [f64]) -> f64 + 'a>,
}

pub fn optimize<T, F>(archetype: &T, params: BiteOptParameters, evaluate: F) -> T
where
    T: Vectorizable + Clone,
    F: Fn(T) -> f64,
{
    let (archetype_vec, archetype_ctx) = archetype.to_vec();
    let dimension = archetype_vec.len() as i32;

    let mut out: Vec<f64> = vec![0.0; dimension as usize];
    let out_ptr: *mut f64 = out.as_mut_ptr();

    let raw_evaluate = |vec: &[f64]| -> f64 {
        let materialized_entity = T::from_vec(vec, &archetype_ctx);
        evaluate(materialized_entity)
    };

    let userdata = Userdata {
        evaluate: Box::new(raw_evaluate),
    };

    let userdata_ptr: *const Userdata = &userdata as *const Userdata;

    unsafe {
        raw::biteopt_optimize(
            out_ptr,
            dimension as c_int,
            userdata_ptr as *const c_void,
            params.lower_bound as c_double,
            params.upper_bound as c_double,
            params.iter as c_int,
            params.depth as c_int,
            params.attc as c_int,
            global_evaluate,
        )
    }

    T::from_vec(&out, &archetype_ctx)
}

extern "C" fn global_evaluate(
    dimension: c_int,
    vec: *const c_double,
    userdata_ptr: *const c_void,
) -> c_double {
    unsafe {
        let userdata_ptr: *const Userdata = userdata_ptr as *const Userdata;
        let vec_slice: &[f64] = std::slice::from_raw_parts(vec as *const f64, dimension as usize);
        let ev = &(*userdata_ptr).evaluate;
        ev(&vec_slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct TwoPoly {
        x: f64,
        y: f64,
    }

    impl Vectorizable for TwoPoly {
        type Context = ();

        fn to_vec(&self) -> (Vec<f64>, Self::Context) {
            (vec![self.x, self.y], ())
        }

        fn from_vec(vec: &[f64], _: &Self::Context) -> Self {
            TwoPoly {
                x: vec[0],
                y: vec[1],
            }
        }
    }

    #[test]
    pub fn test_2polynomial() {
        let mut params = BiteOptParameters::default();
        params.set_lower_bound(-10.0);
        params.set_upper_bound(10.0);
        let optimized = optimize(&TwoPoly { x: 5.0, y: 6.0 }, params, |twopoly| {
            (twopoly.x - 2.0).abs() + (twopoly.y - 8.0).abs()
        });
        assert!((optimized.x - 2.0).abs() < 0.00001);
        assert!((optimized.y - 8.0).abs() < 0.00001);
    }
}
