use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Event};
use crate::modules::live_data_processor::tools::byte_reader;
use crate::modules::live_data_processor::tools::payload_mapper::unit::MapUnit;

pub trait MapEvent {
  fn to_event(&self) -> Result<Event, LiveDataProcessorFailure>;
}

impl MapEvent for [u8] {
  fn to_event(&self) -> Result<Event, LiveDataProcessorFailure> {
    if self.len() != 10 { return Err(LiveDataProcessorFailure::InvalidInput) }
    Ok(Event {
      unit: self[0..9].to_unit()?,
      event_type: self[9]
    })
  }
}