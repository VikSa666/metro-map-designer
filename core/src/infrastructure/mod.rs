//! TODO: DTO (Data Transfer Object) in case models have different form

use serde_json;

use crate::MetroMap;

pub fn serialize(map: &MetroMap) -> Result<String, serde_json::Error> {
    let dto = MetroMap::from(map.clone());
    serde_json::to_string_pretty(&dto)
}

pub fn deserialize(json: &str) -> Result<MetroMap, serde_json::Error> {
    let dto: MetroMap = serde_json::from_str(json)?;
    Ok(dto.into())
}
