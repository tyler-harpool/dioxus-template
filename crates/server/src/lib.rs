#[cfg(feature = "server")]
pub mod db;

pub mod api;

#[cfg(feature = "server")]
pub mod rest;

#[cfg(feature = "server")]
pub mod openapi;

#[cfg(feature = "server")]
pub mod error_convert;

#[cfg(feature = "server")]
pub mod telemetry;

#[cfg(feature = "server")]
pub mod health;

#[cfg(feature = "server")]
pub mod auth;

#[cfg(feature = "server")]
pub mod s3;
