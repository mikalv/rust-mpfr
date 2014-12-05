#![crate_name = "rust-mpfr"]
#![crate_type = "lib"]
#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_double, c_int, c_long, c_void, size_t};
use std::str::from_c_str;

static DEFAULT_PRECISION: c_long = 53;

#[repr(C)]
struct mpfr_struct {
    _mpfr_prec: mpfr_prec_t,
    _mpfr_sign: mpfr_sign_t,
    _mpfr_exp:  mpfr_exp_t,
    _mpfr_d:    *mut c_void
}

impl mpfr_struct {
    pub unsafe fn bare() -> mpfr_struct {
        let mut memory = std::intrinsics::uninit();
        mpfr_init2(&mut memory, DEFAULT_PRECISION);
        memory
    }
}

type mpfr_exp_t  = c_long;
type mpfr_prec_t = c_long;
type mpfr_rnd_t  = c_int;
type mpfr_sign_t = c_int;

#[link(name = "mpfr")]
extern {
    fn mpfr_clear(mpfr: *mut mpfr_struct);
    fn mpfr_get_d(mpfr: *const mpfr_struct, rounding: mpfr_rnd_t) -> c_double;
    fn mpfr_get_si(mpfr: *const mpfr_struct, rounding: mpfr_rnd_t) -> c_long;
    fn mpfr_init2(mpfr: *mut mpfr_struct, precision: mpfr_prec_t);
    fn mpfr_nan_p(mpfr: *const mpfr_struct) -> c_int;
    fn mpfr_set_d(mpfr: *mut mpfr_struct, value: c_double, rounding: mpfr_rnd_t) -> c_int;
    fn mpfr_set_si(mpfr: *mut mpfr_struct, value: c_long, rounding: mpfr_rnd_t) -> c_int;
    fn mpfr_snprintf(buffer: *const c_char, length: size_t, string: *const u8, mpfr: *const mpfr_struct) -> c_int;
}

pub struct MPFR {
    internals: mpfr_struct,
}

impl Drop for MPFR {
    fn drop(&mut self) {
        unsafe {
            mpfr_clear(&mut self.internals);
        }
    }
}

impl MPFR {
    pub fn from_int(value: i64) -> MPFR {
        unsafe {
            let mut mpfr = mpfr_struct::bare();
            mpfr_set_si(&mut mpfr, value, 0);
            MPFR { internals: mpfr }
        }
    }

    pub fn from_float(value: f64) -> MPFR {
        unsafe {
            let mut mpfr = mpfr_struct::bare();
            mpfr_set_d(&mut mpfr, value, 0);
            MPFR { internals: mpfr }
        }
    }
    
    pub fn nan() -> MPFR {
        unsafe {
            let mpfr = mpfr_struct::bare();
            MPFR { internals: mpfr }
        }
    }
    
    pub fn to_int(&self) -> i64 {
        unsafe {
            mpfr_get_si(&self.internals, 0)
        }
    }
    
    pub fn to_float(&self) -> f64 {
        unsafe {
            mpfr_get_d(&self.internals, 0)
        }
    }
    
    pub fn is_nan(&self) -> bool {
        unsafe {
            mpfr_nan_p(&self.internals) != 0
        }
    }
    
    pub fn to_string(&self) -> String {
        unsafe {
        	// todo: this is fixed length! why is fixed length! why!
            let mut vector: Vec<c_char> = Vec::with_capacity(127);
            vector.set_len(127);

            let buffer = vector.as_ptr();
            mpfr_snprintf(buffer, 128, b"%.0Rf\x00".as_ptr(), &self.internals);
            from_c_str(buffer).to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::MPFR;

    #[test]
    fn from_float() {
        assert_eq!(MPFR::from_float(1337f64).to_string(), "1337".to_string())
    }
    
    #[test]
    fn from_int() {
        assert_eq!(MPFR::from_int(1337i64).to_string(), "1337".to_string())
    }
    
    #[test]
    fn to_float() {
        assert_eq!(MPFR::from_float(256f64).to_float(), 256f64)
    }
    
    #[test]
    fn to_int() {
        assert_eq!(MPFR::from_int(128i64).to_int(), 128i64)
    }
    
    #[test]
    fn nanniness() {
        assert!(MPFR::nan().is_nan())
    }
    
    #[test]
    fn anniness() {
        assert!(!MPFR::from_float(27f64).is_nan())
    }
}