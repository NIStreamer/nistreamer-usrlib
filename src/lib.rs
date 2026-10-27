extern crate nistreamer_base;

use nistreamer_base::usrlib_prelude::*;
use std::f64::consts::PI;

usrlib_boilerplate!();

// ========== Examples ==========

/// My linear function:
///     `MyLinFn(t) = slope*t + offs`
#[usr_fn_f64]
pub struct MyLinFn {
    slope: f64,
    offs: f64,
}
impl Calc<f64> for MyLinFn {
    fn calc(&self, t_arr: &[f64], res_arr: &mut[f64]) {
        for (res, &t) in res_arr.iter_mut().zip(t_arr.iter()) {
            *res = self.slope * t + self.offs
        }
    }
}

/// Sine pulse with a Gaussian envelope:
///     `GaussSine(t) = amp(t) * sin(2*PI*freq*t + phase)`
/// where
///     `amp(t) = amp * exp(-(t-t0)^2 / 2*sigma^2)`
#[usr_fn_f64]
pub struct GaussSine {
    t0: f64,
    sigma: f64,
    amp: f64,
    freq: f64,
}
impl Calc<f64> for GaussSine {
    fn calc(&self, t_arr: &[f64], res_arr: &mut [f64]) {
        let exp_denominator = 2.0 * self.sigma.powi(2);
        for (res, &t) in res_arr.iter_mut().zip(t_arr.iter()) {
            let amp = self.amp * f64::exp(
                -(t - self.t0).powi(2) / exp_denominator
            );
            *res = amp * f64::sin(2.0*PI*self.freq * t);
        }
    }
}

/// Boolean constant:
///     val - value
#[usr_fn_bool]
pub struct MyBoolConst {
    val: bool
}
impl Calc<bool> for MyBoolConst {
    fn calc(&self, _t_arr: &[f64], res_arr: &mut [bool]) {
        res_arr.fill(self.val)
    }
}

// ========== Add your waveforms below ==========
