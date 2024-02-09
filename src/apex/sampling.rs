use core::mem::MaybeUninit;

use a653rs::bindings::*;

use super::XngHypervisor;
use crate::bindings::*;

impl ApexSamplingPortP4 for XngHypervisor {
    fn create_sampling_port(
        sampling_port_name: SamplingPortName,
        max_message_size: MessageSize,
        port_direction: PortDirection,
        refresh_period: ApexSystemTime,
    ) -> Result<SamplingPortId, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut port_id = MaybeUninit::uninit();
        const XNG_1_4_CREATE_SAMPLING_PORT_MAGIC_MINIMUM_REFRESH_PERIOD: ApexSystemTime = 2000; // ns
        let refresh_period = if port_direction == PortDirection::Source {
            XNG_1_4_CREATE_SAMPLING_PORT_MAGIC_MINIMUM_REFRESH_PERIOD
        } else {
            refresh_period
        };
        unsafe {
            CREATE_SAMPLING_PORT(
                sampling_port_name.map(|c| c as cty::c_char).as_mut_ptr(),
                max_message_size as MESSAGE_SIZE_TYPE,
                port_direction as PORT_DIRECTION_TYPE,
                refresh_period as SYSTEM_TIME_TYPE,
                port_id.as_mut_ptr(),
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())?;
            Ok(port_id.assume_init() as SamplingPortId)
        }
    }

    fn write_sampling_message(
        sampling_port_id: SamplingPortId,
        message: &[ApexByte],
    ) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            WRITE_SAMPLING_MESSAGE(
                sampling_port_id as SAMPLING_PORT_ID_TYPE,
                message.as_ptr() as *mut _,
                message.len() as MESSAGE_SIZE_TYPE,
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    unsafe fn read_sampling_message(
        sampling_port_id: SamplingPortId,
        message: &mut [ApexByte],
    ) -> Result<(Validity, MessageSize), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut msg_len = MaybeUninit::uninit();
        let mut validity = MaybeUninit::uninit();
        READ_SAMPLING_MESSAGE(
            sampling_port_id as SAMPLING_PORT_ID_TYPE,
            message.as_mut_ptr(),
            msg_len.as_mut_ptr(),
            validity.as_mut_ptr(),
            return_code.as_mut_ptr(),
        );
        ErrorReturnCode::from(return_code.assume_init())?;
        Ok((
            Validity::from_repr(validity.assume_init()).unwrap(),
            msg_len.assume_init() as MessageSize,
        ))
    }
}

impl ApexSamplingPortP1 for XngHypervisor {
    fn get_sampling_port_id(
        sampling_port_name: SamplingPortName,
    ) -> Result<SamplingPortId, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut port_id = MaybeUninit::uninit();
        unsafe {
            GET_SAMPLING_PORT_ID(
                sampling_port_name.map(|c| c as cty::c_char).as_mut_ptr(),
                port_id.as_mut_ptr(),
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())?;
            Ok(port_id.assume_init() as SamplingPortId)
        }
    }

    fn get_sampling_port_status(
        sampling_port_id: SamplingPortId,
    ) -> Result<ApexSamplingPortStatus, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut status = MaybeUninit::uninit();
        unsafe {
            GET_SAMPLING_PORT_STATUS(
                sampling_port_id as SAMPLING_PORT_ID_TYPE,
                status.as_mut_ptr(),
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())?;
            let status = status.assume_init();
            Ok(ApexSamplingPortStatus {
                refresh_period: status.REFRESH_PERIOD,
                max_message_size: status.MAX_MESSAGE_SIZE as MessageSize,
                port_direction: PortDirection::from_repr(status.PORT_DIRECTION).unwrap(),
                last_msg_validity: Validity::from_repr(status.LAST_MSG_VALIDITY).unwrap(),
            })
        }
    }
}
