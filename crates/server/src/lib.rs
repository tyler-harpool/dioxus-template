#[cfg(feature = "server")]
pub mod db;

pub mod api;

#[cfg(feature = "server")]
pub mod rest;

#[cfg(feature = "server")]
pub mod openapi;
