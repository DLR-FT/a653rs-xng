use core::mem::MaybeUninit;

use a653rs::bindings::*;

use super::XngHypervisor;
use crate::bindings::*;

impl ApexTimeP4 for XngHypervisor {
    fn periodic_wait() -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            PERIODIC_WAIT(return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    fn get_time() -> ApexSystemTime {
        let mut time = MaybeUninit::uninit();
        unsafe {
            GET_TIME(time.as_mut_ptr(), MaybeUninit::uninit().as_mut_ptr());
            time.assume_init() as ApexSystemTime
        }
    }
}

impl ApexTimeP1 for XngHypervisor {
    fn timed_wait(delay_time: ApexSystemTime) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            TIMED_WAIT(delay_time, return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    fn replenish(budget_time: ApexSystemTime) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            REPLENISH(budget_time, return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())
        }
    }
}
