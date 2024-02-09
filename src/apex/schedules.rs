use core::mem::MaybeUninit;

use a653rs::bindings::*;

use super::XngHypervisor;
use crate::bindings::*;

impl ApexScheduleP2 for XngHypervisor {
    fn set_module_schedule(schedule_id: ScheduleId) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            SET_MODULE_SCHEDULE(schedule_id as SCHEDULE_ID_TYPE, return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    fn get_module_schedule_status() -> Result<ApexScheduleStatus, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut status = MaybeUninit::uninit();
        unsafe {
            GET_MODULE_SCHEDULE_STATUS(status.as_mut_ptr(), return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())?;
            let status = status.assume_init();
            Ok(ApexScheduleStatus {
                time_of_last_schedule_switch: status.TIME_OF_LAST_SCHEDULE_SWITCH,
                current_schedule: status.CURRENT_SCHEDULE as ScheduleId,
                next_schedule: status.NEXT_SCHEDULE as ScheduleId,
            })
        }
    }

    fn get_module_schedule_id(schedule_name: ScheduleName) -> Result<ScheduleId, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut schedule_id = MaybeUninit::uninit();
        unsafe {
            GET_MODULE_SCHEDULE_ID(
                schedule_name.map(|c| c as cty::c_char).as_mut_ptr(),
                schedule_id.as_mut_ptr(),
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())?;
            Ok(schedule_id.assume_init() as ScheduleId)
        }
    }
}
