#![allow(non_camel_case_types)]
#![feature(libc)]
#![feature(core)]

extern crate libc;

use libc::{c_char, c_double, c_int, c_long, c_void, size_t};
use std::cmp::Ordering;
use std::str;
use std::ffi::{CStr};
use std::string::{ToString};
use std::ops::{Add, Sub, Mul, Div};

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
    fn mpfr_add(result: *mut mpfr_struct, a: *const mpfr_struct, b: *const mpfr_struct, rounding: mpfr_rnd_t) -> c_int;
    fn mpfr_clear(mpfr: *mut mpfr_struct);
    fn mpfr_cmp_d(mpfr: *const mpfr_struct, other: c_double) -> c_int;
    fn mpfr_cmp_si(mpfr: *const mpfr_struct, other: c_long) -> c_int;
    fn mpfr_cmp(mpfr: *const mpfr_struct, other: *const mpfr_struct) -> c_int;
    fn mpfr_div(result: *mut mpfr_struct, a: *const mpfr_struct, b: *const mpfr_struct, rounding: mpfr_rnd_t) -> c_int;
    fn mpfr_equal_p(mpfr: *const mpfr_struct, other: *const mpfr_struct) -> c_int;
    fn mpfr_get_prec(mpfr: *const mpfr_struct) -> c_long;
    fn mpfr_get_d(mpfr: *const mpfr_struct, rounding: mpfr_rnd_t) -> c_double;
    fn mpfr_get_si(mpfr: *const mpfr_struct, rounding: mpfr_rnd_t) -> c_long;
    fn mpfr_greater_p(mpfr: *const mpfr_struct, other: *const mpfr_struct) -> c_int;
    fn mpfr_greaterequal_p(mpfr: *const mpfr_struct, other: *const mpfr_struct) -> c_int;
    fn mpfr_inf_p(mpfr: *const mpfr_struct) -> c_int;
    fn mpfr_init2(mpfr: *mut mpfr_struct, precision: mpfr_prec_t);
    fn mpfr_less_p(mpfr: *const mpfr_struct, other: *const mpfr_struct) -> c_int;
    fn mpfr_lessequal_p(mpfr: *const mpfr_struct, other: *const mpfr_struct) -> c_int;
    fn mpfr_lessgreater_p(mpfr: *const mpfr_struct, other: *const mpfr_struct) -> c_int;
    fn mpfr_mul(result: *mut mpfr_struct, a: *const mpfr_struct, b: *const mpfr_struct, rounding: mpfr_rnd_t) -> c_int;
    fn mpfr_nan_p(mpfr: *const mpfr_struct) -> c_int;
    fn mpfr_number_p(mpfr: *const mpfr_struct) -> c_int;
    fn mpfr_regular_p(mpfr: *const mpfr_struct) -> c_int;
    fn mpfr_set_d(mpfr: *mut mpfr_struct, value: c_double, rounding: mpfr_rnd_t) -> c_int;
    fn mpfr_set_inf(mpfr: *mut mpfr_struct, sign: c_int);
    fn mpfr_set_nan(mpfr: *mut mpfr_struct);
    fn mpfr_set_si(mpfr: *mut mpfr_struct, value: c_long, rounding: mpfr_rnd_t) -> c_int;
    fn mpfr_set_zero(mpfr: *mut mpfr_struct, sign: c_int);
    fn mpfr_snprintf(buffer: *const c_char, length: size_t, string: *const u8, mpfr: *const mpfr_struct) -> c_int;
    fn mpfr_sub(result: *mut mpfr_struct, a: *const mpfr_struct, b: *const mpfr_struct, rounding: mpfr_rnd_t) -> c_int;
    fn mpfr_unordered_p(mpfr: *const mpfr_struct, other: *const mpfr_struct) -> c_int;
    fn mpfr_zero_p(mpfr: *const mpfr_struct) -> c_int;
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
            let mut mpfr = mpfr_struct::bare();
            mpfr_set_nan(&mut mpfr);
            MPFR { internals: mpfr }
        }
    }
    
    pub fn infinity() -> MPFR {
        unsafe {
            let mut mpfr = mpfr_struct::bare();
            mpfr_set_inf(&mut mpfr, 1);
            MPFR { internals: mpfr }
        }
    }

    pub fn negative_infinity() -> MPFR {
        unsafe {
            let mut mpfr = mpfr_struct::bare();
            mpfr_set_inf(&mut mpfr, -1);
            MPFR { internals: mpfr }
        }
    }
    
    pub fn zero() -> MPFR {
        unsafe {
            let mut mpfr = mpfr_struct::bare();
            mpfr_set_zero(&mut mpfr, 1);
            MPFR { internals: mpfr }
        }
    }

    pub fn negative_zero() -> MPFR {
        unsafe {
            let mut mpfr = mpfr_struct::bare();
            mpfr_set_zero(&mut mpfr, 1);
            MPFR { internals: mpfr }
        }
    }
    
    pub fn to_int(&self) -> i64 {
        unsafe { mpfr_get_si(&self.internals, 0) }
    }
    
    pub fn to_float(&self) -> f64 {
        unsafe { mpfr_get_d(&self.internals, 0) }
    }
    
    pub fn is_nan(&self) -> bool {
        unsafe { mpfr_nan_p(&self.internals) != 0 }
    }
    
    pub fn is_infinity(&self) -> bool {
        unsafe { mpfr_inf_p(&self.internals) != 0 }
    }

    pub fn is_zero(&self) -> bool {
        unsafe { mpfr_zero_p(&self.internals) != 0 }
    }
    
    pub fn is_number(&self) -> bool {
        unsafe { mpfr_number_p(&self.internals) != 0 }
    }

    pub fn is_regular(&self) -> bool {
        unsafe { mpfr_regular_p(&self.internals) != 0 }
    }

    
    pub fn is_equal_to_int(&self, value: i64) -> bool {
        unsafe { mpfr_cmp_si(&self.internals, value) == 0 }
    }

    pub fn is_equal_to_float(&self, value: f64) -> bool {
        unsafe { mpfr_cmp_d(&self.internals, value) == 0 }
    }

}

impl ToString for MPFR {
    fn to_string(&self) -> String {
        unsafe {
            let prec : c_long = mpfr_get_prec(&self.internals)/8;
            let buff : Vec<c_char> = Vec::with_capacity(prec as usize);
            mpfr_snprintf(buff.as_ptr(),
                          prec as size_t,
                          b"%.0Rf".as_ptr(),
                          &self.internals);
            let s = CStr::from_ptr(buff.as_ptr());
            str::from_utf8(s.to_bytes()).unwrap().to_string()
        }
    }
}

impl PartialEq for MPFR {
    fn eq(&self, other: &MPFR) -> bool {
        unsafe {
            mpfr_equal_p(&self.internals, &other.internals) != 0
        }
    }
    
    fn ne(&self, other: &MPFR) -> bool {
        unsafe {
            mpfr_lessgreater_p(&self.internals, &other.internals) != 0
        }
    }
}

impl PartialOrd for MPFR {
    fn partial_cmp(&self, other: &MPFR) -> Option<Ordering> {
        let smoke_test = unsafe { mpfr_unordered_p(&self.internals, &other.internals) };
        if smoke_test != 0 {
            return None;
        }
        
        let result = unsafe { mpfr_cmp(&self.internals, &other.internals) };
        Some(if result == 0 {
            Ordering::Equal
        }
        else if result < 0 {
            Ordering::Less
        }
        else {
            Ordering::Greater
        })
    }
    
    fn lt(&self, other: &MPFR) -> bool {
        unsafe { mpfr_less_p(&self.internals, &other.internals) != 0 }
    }

    fn le(&self, other: &MPFR) -> bool {
        unsafe { mpfr_lessequal_p(&self.internals, &other.internals) != 0 }
    }
    
    fn gt(&self, other: &MPFR) -> bool {
        unsafe { mpfr_greater_p(&self.internals, &other.internals) != 0 }
    }

    fn ge(&self, other: &MPFR) -> bool {
        unsafe { mpfr_greaterequal_p(&self.internals, &other.internals) != 0 }
    }
}

impl Add for MPFR {
    type Output = MPFR;
    
    fn add(self, other: MPFR) -> MPFR {
        unsafe {
            let mut result = mpfr_struct::bare();
            mpfr_add(&mut result, &self.internals, &other.internals, 0);
            MPFR { internals: result }
        }
    }
}

impl Sub for MPFR {
    type Output = MPFR;

    fn sub(self, other: MPFR) -> MPFR {
        unsafe {
            let mut result = mpfr_struct::bare();
            mpfr_sub(&mut result, &self.internals, &other.internals, 0);
            MPFR { internals: result }
        }
    }
}

impl Mul for MPFR {
    type Output = MPFR;
    
    fn mul(self, other: MPFR) -> MPFR {
        unsafe {
            let mut result = mpfr_struct::bare();
            mpfr_mul(&mut result, &self.internals, &other.internals, 0);
            MPFR { internals: result }
        }
    }
}

impl Div for MPFR {
    type Output = MPFR;

    fn div(self, other: MPFR) -> MPFR {
        unsafe {
            let mut result = mpfr_struct::bare();
            mpfr_div(&mut result, &self.internals, &other.internals, 0);
            MPFR { internals: result }
        }
    }
}

#[cfg(test)]
mod test {
    pub use super::MPFR;

    #[test]
    fn from_float() {
        assert_eq!(MPFR::from_float(1337f64).to_string(), "1337".to_string());
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
    
    mod cmp {
        use super::MPFR;
        
        #[test]
        fn eq() {
            assert!(MPFR::from_float(256f64) == MPFR::from_int(256i64))
        }

        #[test]
        fn eq_2() {
            assert!(!(MPFR::from_float(256f64) == MPFR::from_int(128i64)))
        }
    
        #[test]
        fn ne() {
            assert!(MPFR::from_int(7i64) != MPFR::from_int(77i64))
        }

        #[test]
        fn ne_2() {
            assert!(!(MPFR::from_int(7i64) != MPFR::from_int(7i64)))
        }
        
        #[test]
        fn less() {
            assert!(MPFR::from_int(7i64) < MPFR::from_int(8i64))
        }
        
        #[test]
        fn less_2() {
            assert!(!(MPFR::from_int(7i64) < MPFR::from_int(6i64)))
        }

        #[test]
        fn less_equal() {
            assert!(MPFR::from_int(7i64) <= MPFR::from_int(8i64))
        }

        #[test]
        fn less_equal_2() {
            assert!(MPFR::from_int(7i64) <= MPFR::from_int(7i64))
        }

        #[test]
        fn greater() {
            assert!(MPFR::from_int(7i64) > MPFR::from_int(6i64))
        }
        
        #[test]
        fn greater_2() {
            assert!(!(MPFR::from_int(7i64) > MPFR::from_int(8i64)))
        }

        #[test]
        fn greater_equal() {
            assert!(MPFR::from_int(7i64) >= MPFR::from_int(6i64))
        }

        #[test]
        fn greater_equal_2() {
            assert!(MPFR::from_int(7i64) >= MPFR::from_int(7i64))
        }
        
        #[test]
        fn equal_to_int() {
            assert!(MPFR::from_int(8i64).is_equal_to_int(8i64))
        }

        #[test]
        fn equal_to_int_2() {
            assert!(!(MPFR::from_int(8i64).is_equal_to_int(9i64)))
        }
        
        #[test]
        fn equal_to_float() {
            assert!(MPFR::from_int(8i64).is_equal_to_float(8f64))
        }

        #[test]
        fn equal_to_float_2() {
            assert!(!(MPFR::from_int(8i64).is_equal_to_float(9f64)))
        }       
    }   

    mod type_cmp {
        use super::MPFR;
        
        #[test]
        fn nanniness() {
            assert!(MPFR::nan().is_nan())
        }
    
        #[test]
        fn anniness() {
            assert!(!MPFR::from_float(27f64).is_nan())
        }
        
        #[test]
        fn positive_infinity() {
            assert!(MPFR::infinity().is_infinity())
        }
        
        #[test]
        fn negative_infinity() {
            assert!(MPFR::negative_infinity().is_infinity())
        }
    
        #[test]
        fn positive_zero() {
            assert!(MPFR::zero().is_zero())
        }
        
        #[test]
        fn negative_zero() {
            assert!(MPFR::negative_zero().is_zero())
        }
        
        #[test]
        fn number() {
            assert!(MPFR::zero().is_number())
        }
        
        #[test]
        fn regular() {
            assert!(!(MPFR::zero().is_regular()))
        }
    }
    
    mod arithmetic {
        use super::MPFR;
        
        #[test]
        fn add() {
            assert!(MPFR::from_int(3i64) + MPFR::from_int(4i64) == MPFR::from_int(7i64))
        }
        
        #[test]
        fn sub() {
            assert!(MPFR::from_int(10i64) - MPFR::from_float(3.5f64) == MPFR::from_float(6.5f64))
        }
    
        #[test]
        fn mul() {
            assert!(MPFR::from_int(3i64) * MPFR::from_int(4i64) == MPFR::from_int(12i64))
        }
        
        #[test]
        fn div() {
            assert!(MPFR::from_int(5i64) / MPFR::from_int(2i64) == MPFR::from_float(2.5f64))
        }
    }
}
