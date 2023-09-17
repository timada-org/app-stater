mod aggregate;
mod command;
mod event;
mod projection;

pub use aggregate::*;
pub use command::*;
pub use event::*;
pub use projection::*;

#[cfg(feature = "proto")]
tonic::include_proto!("starter");
