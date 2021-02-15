use sqlx::{self, types::chrono, FromRow, Type};
use std::{convert::TryFrom, time::SystemTime};
#[derive(FromRow, Debug)]
pub struct User {
  pub id: i64,
  pub name: String,
  pub active: Active,
  pub created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
}

impl Active {
  pub fn into_i32(&self) -> i32 {
    match self {
      Active::OffLine => 1,
      Active::Active => 2,
      Active::NotSeen => 3,
      Active::Unspecified => 0,
    }
  }
}

pub trait ChronoToProstTime {
  fn into_prost_timestamp(&self) -> prost_types::Timestamp;
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

impl ChronoToProstTime for chrono::DateTime<chrono::Utc> {
  fn into_prost_timestamp(&self) -> prost_types::Timestamp {
    // https://docs.rs/prost-types/0.2.3/prost_types/struct.Timestamp.html
    let prost_timestamp = prost_types::Timestamp {
      // https://docs.rs/sqlx/0.4.0-beta.1/sqlx/types/chrono/struct.DateTime.html
      nanos: self.timestamp_nanos() as i32,
      seconds: self.timestamp(),
    };
    println!(
      "converted time->{:?}, psqltime->{:?}",
      SystemTime::try_from(prost_timestamp.clone()),
      self
    );
    prost_timestamp
  }
}

#[derive(Type, Debug)]
#[sqlx(type_name = "active", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Active {
  Unspecified,
  OffLine,
  Active,
  NotSeen,
}

impl std::default::Default for Active {
  fn default() -> Self {
    Active::Unspecified
  }
}
