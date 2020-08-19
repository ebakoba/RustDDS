use serde::{Serialize, Deserialize};
use super::parameter_id::ParameterId;
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct BuiltinEndpointSet {
  value: u32,
}

impl BuiltinEndpointSet {
  pub const DISC_BUILTIN_ENDPOINT_PARTICIPANT_ANNOUNCER: u32 = 0x00000001;
  pub const DISC_BUILTIN_ENDPOINT_PARTICIPANT_DETECTOR: u32 = 0x000000002;
  pub const DISC_BUILTIN_ENDPOINT_PUBLICATIONS_ANNOUNCER: u32 = 0x00000004;
  pub const DISC_BUILTIN_ENDPOINT_PUBLICATIONS_DETECTOR: u32 = 0x00000008;
  pub const DISC_BUILTIN_ENDPOINT_SUBSCRIPTIONS_ANNOUNCER: u32 = 0x00000010;
  pub const DISC_BUILTIN_ENDPOINT_SUBSCRIPTIONS_DETECTOR: u32 = 0x00000020;

  pub const DISC_BUILTIN_ENDPOINT_TOPICS_ANNOUNCER: u32 = 0x08000000;
  pub const DISC_BUILTIN_ENDPOINT_TOPICS_DETECTOR: u32 = 0x10000000;

  pub fn from(val: u32) -> BuiltinEndpointSet {
    BuiltinEndpointSet { value: val }
  }
}

#[derive(Serialize, Deserialize)]
pub struct BuiltinEndpointSetData {
  parameter_id: ParameterId,
  parameter_length: u16,
  builtin_endpoint_set: BuiltinEndpointSet,
}

impl BuiltinEndpointSetData {
  pub fn from(
    builtin_endpoint_set: &BuiltinEndpointSet,
    parameter_id: &ParameterId,
  ) -> BuiltinEndpointSetData {
    BuiltinEndpointSetData {
      parameter_id: parameter_id.clone(),
      parameter_length: 4,
      builtin_endpoint_set: builtin_endpoint_set.clone(),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct BuiltinEndpointQos {
  value: u32,
}

impl BuiltinEndpointQos {
  pub const BEST_EFFORT_PARTICIPANT_MESSAGE_DATA_READER: u32 = 0x00000001;
}

#[derive(Serialize, Deserialize)]
pub struct BuiltinEndpointQosData {
  parameter_id: ParameterId,
  parameter_length: u16,
  builtin_endpoint_qos: BuiltinEndpointQos,
}

impl BuiltinEndpointQosData {
  pub fn from(builtin_endpoint_qos: &BuiltinEndpointQos) -> BuiltinEndpointQosData {
    BuiltinEndpointQosData {
      parameter_id: ParameterId::PID_BUILTIN_ENDPOINT_QOS,
      parameter_length: 4,
      builtin_endpoint_qos: builtin_endpoint_qos.clone(),
    }
  }
}
