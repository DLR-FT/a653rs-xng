use core::mem::MaybeUninit;

use a653rs::bindings::*;

use super::XngHypervisor;
use crate::bindings::*;

impl ApexQueuingPortP4 for XngHypervisor {
    fn create_queuing_port(
        queuing_port_name: QueuingPortName,
        max_message_size: MessageSize,
        max_nb_message: MessageRange,
        port_direction: PortDirection,
        queuing_discipline: QueuingDiscipline,
    ) -> Result<QueuingPortId, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut port_id = MaybeUninit::uninit();
        unsafe {
            CREATE_QUEUING_PORT(
                queuing_port_name.map(|c| c as cty::c_char).as_mut_ptr(),
                max_message_size as MESSAGE_SIZE_TYPE,
                max_nb_message as MESSAGE_RANGE_TYPE,
                port_direction as PORT_DIRECTION_TYPE,
                queuing_discipline as QUEUING_DISCIPLINE_TYPE,
                port_id.as_mut_ptr(),
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())?;
            Ok(port_id.assume_init() as QueuingPortId)
        }
    }

    fn send_queuing_message(
        queuing_port_id: QueuingPortId,
        message: &[ApexByte],
        time_out: ApexSystemTime,
    ) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            SEND_QUEUING_MESSAGE(
                queuing_port_id as QUEUING_PORT_ID_TYPE,
                message.as_ptr() as *mut _,
                message.len() as MESSAGE_SIZE_TYPE,
                time_out,
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    unsafe fn receive_queuing_message(
        queuing_port_id: QueuingPortId,
        time_out: ApexSystemTime,
        message: &mut [ApexByte],
    ) -> Result<(MessageSize, QueueOverflow), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut msg_len = MaybeUninit::uninit();
        RECEIVE_QUEUING_MESSAGE(
            queuing_port_id as QUEUING_PORT_ID_TYPE,
            time_out,
            message.as_mut_ptr(),
            msg_len.as_mut_ptr(),
            return_code.as_mut_ptr(),
        );
        let error_code = ErrorReturnCode::from(return_code.assume_init());
        let mut overflowed = false;
        if let Err(e) = error_code {
            if ErrorReturnCode::InvalidConfig == e {
                overflowed = true;
            } else {
                error_code?;
            }
        };
        Ok((msg_len.assume_init() as MessageSize, overflowed))
    }

    fn get_queuing_port_status(
        queuing_port_id: QueuingPortId,
    ) -> Result<QueuingPortStatus, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut status = MaybeUninit::uninit();
        unsafe {
            GET_QUEUING_PORT_STATUS(
                queuing_port_id as QUEUING_PORT_ID_TYPE,
                status.as_mut_ptr(),
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())?;
            let status = status.assume_init();
            Ok(QueuingPortStatus {
                nb_message: status.NB_MESSAGE as MessageRange,
                max_nb_message: status.MAX_NB_MESSAGE as MessageRange,
                max_message_size: status.MAX_MESSAGE_SIZE as MessageSize,
                port_direction: PortDirection::from_repr(status.PORT_DIRECTION).unwrap(),
                waiting_processes: status.WAITING_PROCESSES as WaitingRange,
            })
        }
    }

    fn clear_queuing_port(queuing_port_id: QueuingPortId) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            CLEAR_QUEUING_PORT(
                queuing_port_id as QUEUING_PORT_ID_TYPE,
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())
        }
    }
}

impl ApexQueuingPortP1 for XngHypervisor {
    fn get_queuing_port_id(
        queuing_port_name: QueuingPortName,
    ) -> Result<QueuingPortId, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut port_id = MaybeUninit::uninit();
        unsafe {
            GET_QUEUING_PORT_ID(
                queuing_port_name.map(|c| c as cty::c_char).as_mut_ptr(),
                port_id.as_mut_ptr(),
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())?;
            Ok(port_id.assume_init() as QueuingPortId)
        }
    }
}
