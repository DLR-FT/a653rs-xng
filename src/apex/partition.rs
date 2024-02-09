use core::mem::MaybeUninit;

use a653rs::bindings::*;

use super::XngHypervisor;
use crate::bindings::*;

impl ApexPartitionP4 for XngHypervisor {
    fn get_partition_status() -> ApexPartitionStatus {
        let mut status = MaybeUninit::uninit();
        unsafe {
            GET_PARTITION_STATUS(status.as_mut_ptr(), MaybeUninit::uninit().as_mut_ptr());
            let status = status.assume_init();
            ApexPartitionStatus {
                period: status.PERIOD,
                duration: status.DURATION,
                identifier: status.IDENTIFIER as PartitionId,
                lock_level: status.LOCK_LEVEL as LockLevel,
                operating_mode: OperatingMode::from_repr(status.OPERATING_MODE).unwrap(),
                start_condition: StartCondition::from_repr(status.START_CONDITION).unwrap(),
                // TODO check whether this is correct
                // this would normally be
                // status.NUM_ASSIGNED_CORES as NumCores
                // but since theses headers where created for xng-monocore,
                // the number is always 1
                num_assigned_cores: 1,
            }
        }
    }

    fn set_partition_mode(operating_mode: OperatingMode) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            SET_PARTITION_MODE(
                operating_mode as OPERATING_MODE_TYPE,
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())
        }
    }
}
