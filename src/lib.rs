//! Gudgeon: Autonomous Surface Vessel Navigation & Control Engine
//! Written in LOLCODE, Rust, and Rhai.
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

#![allow(
    clippy::too_many_arguments,
    clippy::missing_safety_doc,
    clippy::uninlined_format_args,
    clippy::manual_range_contains
)]

pub mod c_api;
pub mod colregs;
pub mod lol;
pub mod navigation;
pub mod propulsion;
pub mod rhai_bridge;
pub mod sensors;
pub mod telemetry;
pub mod vessel;
