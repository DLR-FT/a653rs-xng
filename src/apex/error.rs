use core::mem::MaybeUninit;

use apex_rs::bindings::*;
use log::trace;

use super::XngHypervisor;
use crate::bindings::*;

impl ApexErrorP4 for XngHypervisor {
    fn report_application_message<L: Locked>(message: &[ApexByte]) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            REPORT_APPLICATION_MESSAGE(
                message.as_ptr() as *mut _,
                message.len() as MESSAGE_SIZE_TYPE,
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    fn raise_application_error<L: Locked>(
        error_code: ErrorCode,
        message: &[ApexByte],
    ) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            RAISE_APPLICATION_ERROR(
                error_code as ERROR_CODE_TYPE,
                message.as_ptr() as *mut _,
                message.len() as MESSAGE_SIZE_TYPE,
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())
        }
    }
}

impl ApexErrorP1 for XngHypervisor {
    fn create_error_handler<L: Locked>(
        entry_point: SystemAddress,
        stack_size: StackSize,
    ) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            CREATE_ERROR_HANDLER(
                entry_point as SYSTEM_ADDRESS_TYPE,
                stack_size as STACK_SIZE_TYPE,
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    fn get_error_status<L: Locked>() -> Result<ErrorStatus, ErrorReturnCode> {
        todo!()
    }

    fn configure_error_handler<L: Locked>(
        _concurrency_control: ErrorHandlerConcurrencyControl,
        _processor_core_id: ProcessorCoreId,
    ) -> Result<(), ErrorReturnCode> {
        // TODO make this functional once the function lands in LithOS
        /*
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            CONFIGURE_ERROR_HANDLER(
                concurrency_control as ERROR_HANDLER_CONCURRENCY_CONTROL_TYPE,
                processor_core_id as PROCESSOR_CORE_ID_TYPE,
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())
        }
        */
        Err(ErrorReturnCode::NotAvailable)
    }
}
