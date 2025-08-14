extern crate nistreamer_base;

use nistreamer_base::usrlib_prelude::*;
use std::f64::consts::PI;

usrlib_boilerplate!();

/// Linear function:
///     a: slope
///     b: offset
#[usr_fn_f64]
pub struct MyLinFn {
    a: f64,
    b: f64,
}
impl Calc<f64> for MyLinFn {
    fn calc(&self, t_arr: &[f64], res_arr: &mut[f64]) {
        for (res, &t) in res_arr.iter_mut().zip(t_arr.iter()) {
            *res = self.a * t + self.b
        }
    }
}

/// Sine function:
///     amp - amplitude (in Volts)
///     freq - linear frequency (in Hz)
///     phase - absolute phase (in radians)
///     offs - offset (in Volts)
#[usr_fn_f64(amp, freq, phase=0.0, offs=0.0)]
pub struct MySine {
    amp: f64,
    freq: f64,
    phase: f64,
    offs: f64,
}
impl Calc<f64> for MySine {
    fn calc(&self, t_arr: &[f64], res_arr: &mut[f64]) {
        for (res, &t) in res_arr.iter_mut().zip(t_arr.iter()) {
            *res = self.offs + self.amp * f64::sin(2.0*PI * self.freq * t + self.phase)
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
