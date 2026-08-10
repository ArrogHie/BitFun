//! Coordination layer
//!
//! Top-level component that integrates all subsystems

mod background_outcomes;
mod coordination_store;
pub mod coordinator;
pub mod scheduler;
pub mod state_manager;
mod subagent_instance;
pub mod turn_outcome;
mod turn_settlement;

pub use coordinator::*;
pub use scheduler::*;
pub use state_manager::*;
pub use turn_outcome::*;

pub(crate) use background_outcomes::{
    BackgroundSubagentOutcome, BackgroundSubagentOutcomeStore, BackgroundSubagentWaitMode,
    BackgroundSubagentWaitResult,
};

// Wired into the Coordinator as the persistent subagent instance registry.
pub(crate) use subagent_instance::{SubagentInstance, SubagentInstanceRegistry};
// Referenced by coordinator tests; the lib call path only reads the status
// through the registry API.
#[allow(unused_imports)]
pub(crate) use subagent_instance::SubagentInstanceStatus;

pub use coordinator::get_global_coordinator;
pub use scheduler::get_global_scheduler;
