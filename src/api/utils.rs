use crate::client::Timestep;

#[must_use]
pub fn round_to_previous_timestamp(timestep: Timestep, timestamp: i64) -> i64 {
    let timestep_secs: i64 = timestep.into();
    (timestamp / timestep_secs) * timestep_secs
}
