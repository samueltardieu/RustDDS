#[cfg(test)]
use bytes::Bytes;

use crate::{
  dds::traits::key::KeyHash,
  messages::submessages::submessage_elements::serialized_payload::SerializedPayload,
};
//use crate::messages::submessages::submessages::RepresentationIdentifier;
use crate::structure::cache_change::ChangeKind;

// DDSData represets a serialized data sample with metadata

#[derive(Debug, PartialEq, Clone)]
// Contents of a DATA submessage or several DATAFRAG submessages. This is either
// a new sample, or key, or a key hash. The latter two are used to indicate
// dispose or unregister.
pub enum DDSData {
  Data {
    serialized_payload: SerializedPayload,
  },
  // DataFrags {
  //   // Each DATAFRAG specifies RepresentationIdentifier, but we assume they are the same.
  //   // Otherwise, decoding would be exceedingly confusing.
  //   representation_identifier: RepresentationIdentifier,
  //   // The payload is stored as a Vec of Bytes buffers.
  //   // Deserializer should concateneate these and deserialize.
  //   bytes_frags: Vec<Bytes>,
  // },
  DisposeByKey {
    change_kind: ChangeKind,
    key: SerializedPayload,
  },
  DisposeByKeyHash {
    change_kind: ChangeKind,
    key_hash: KeyHash,
  },
}

impl DDSData {
  pub fn new(serialized_payload: SerializedPayload) -> Self {
    Self::Data { serialized_payload }
  }
  pub fn new_disposed_by_key(change_kind: ChangeKind, key: SerializedPayload) -> Self {
    Self::DisposeByKey { change_kind, key }
  }

  pub fn new_disposed_by_key_hash(change_kind: ChangeKind, key_hash: KeyHash) -> Self {
    Self::DisposeByKeyHash {
      change_kind,
      key_hash,
    }
  }

  #[allow(dead_code)] // Why is this not used?
  pub fn change_kind(&self) -> ChangeKind {
    match self {
      DDSData::Data {..} /*| DDSData::DataFrags {..}*/ => ChangeKind::Alive,
      DDSData::DisposeByKey { change_kind, ..} | DDSData::DisposeByKeyHash { change_kind, .. }  => *change_kind,
    }
  }

  // pub fn serialized_payload(&self) -> Option<&SerializedPayload> {
  //   match &self {
  //     DDSData::Data { serialized_payload } => Some( serialized_payload ),
  //     DDSData::DisposeByKey { key , ..} => Some( key ),
  //     DDSData::DisposeByKeyHash {..} => None,
  //   }
  // }

  // What is the serialized size of this?
  pub fn payload_size(&self) -> usize {
    match self {
      DDSData::Data { serialized_payload } => serialized_payload.len(),
      DDSData::DisposeByKey { key, .. } => key.len(),
      DDSData::DisposeByKeyHash { .. } => 16,
      // This is a fundamental constant of the RTPS
      // specification v2.5 Section 9.6.4.8 KeyHash (PID_KEY_HASH)
    }
  }

  #[cfg(test)]
  pub fn data(&self) -> Option<Bytes> {
    match &self {
      DDSData::Data { serialized_payload } => Some(serialized_payload.value.clone()),
      // DDSData::DataFrags { _representation_identifier, bytes_frags } =>
      //   Some(   ) ,
      DDSData::DisposeByKey { key, .. } => Some(key.value.clone()),
      DDSData::DisposeByKeyHash { .. } => None,
    }
  }
}
