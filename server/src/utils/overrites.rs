use crate::models::Active;
use prost_types::Timestamp;
use sqlx::types::chrono;
pub trait ChronoToProstTime {
  fn into_prost_timestamp(&self) -> Timestamp;
}

impl ChronoToProstTime for chrono::DateTime<chrono::Utc> {
  fn into_prost_timestamp(&self) -> Timestamp {
    // https://docs.rs/prost-types/0.2.3/prost_types/struct.Timestamp.html
    let prost_timestamp = Timestamp {
      // https://docs.rs/sqlx/0.4.0-beta.1/sqlx/types/chrono/struct.DateTime.html
      nanos: self.timestamp_nanos() as i32,
      seconds: self.timestamp(),
    };
    prost_timestamp
  }
}

pub trait ProtoActiveOverrite {
  fn into_enum(&self) -> Active;
}

impl ProtoActiveOverrite for i32 {
  fn into_enum(&self) -> Active {
    match self {
      1 => Active::OffLine,
      2 => Active::Active,
      3 => Active::NotSeen,
      _ => Active::Unspecified,
    }
  }
}
