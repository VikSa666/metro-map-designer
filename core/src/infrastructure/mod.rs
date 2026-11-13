//! DTO (Data Transfer Object) in case models have different form

use serde_json;

use crate::MetroMap;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StationDto {
    pub id: String,
    pub name: String,
    pub svg_coordinates: SvgPointDto,
    pub label_svg_coordinates: Option<SvgPointDto>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineDto {
    pub id: String,
    pub name: String,
    pub color: String,
    pub routes: Vec<RouteDto>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RouteDto {
    pub id: String,
    pub name: Option<String>,
    pub tracks: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TrackDto {
    pub id: String,
    pub from: String,
    pub to: String,
    pub edges: Vec<SvgPointDto>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SvgPointDto {
    pub x: f32,
    pub y: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MetroMapDto {
    pub id: String,
    pub stations: Vec<StationDto>,
    pub lines: Vec<LineDto>,
    pub tracks: Vec<TrackDto>,
}

pub fn serialize(map: &MetroMap) -> Result<String, serde_json::Error> {
    let dto = MetroMapDto::from(map.clone());
    serde_json::to_string_pretty(&dto)
}

pub fn deserialize(json: &str) -> Result<MetroMap, serde_json::Error> {
    let dto: MetroMapDto = serde_json::from_str(json)?;
    Ok(dto.into())
}
