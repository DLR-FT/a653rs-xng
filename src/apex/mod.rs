/// [apex_rs::bindings::ApexErrorP1] and [apex_rs::bindings::ApexErrorP4] bindings
pub mod error;
/// [apex_rs::bindings::ApexPartitionP4] bindings
pub mod partition;
/// [apex_rs::bindings::ApexProcessP1] and [apex_rs::bindings::ApexProcessP4] bindings
pub mod process;
/// [apex_rs::bindings::ApexQueuingPortP1] and [apex_rs::bindings::ApexQueuingPortP4] bindings
pub mod queuing;
/// [apex_rs::bindings::ApexSamplingPortP1] and [apex_rs::bindings::ApexSamplingPortP4] bindings
pub mod sampling;
/// [apex_rs::bindings::ApexScheduleP2] bindings
pub mod schedules;
/// [apex_rs::bindings::ApexTimeP1] and [apex_rs::bindings::ApexTimeP4] bindings
pub mod time;

/// Static struct representing a Xng Hypervisor
#[derive(Debug, Clone, Copy)]
pub struct XngHypervisor;
