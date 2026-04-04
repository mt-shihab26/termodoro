//! Orivo is a terminal-first pomodoro and todo application.
//!
//! The library crate is organised around a small set of runtime layers:
//!
//! - `cmds` provides the CLI command surface used by the binary entrypoint.
//! - `domains` contains top-level runtime orchestration such as the TUI app loop
//!   and development data seeding.
//! - `tabs`, `states`, `widgets`, and `workers` implement the interactive
//!   terminal UI, background event handling, and state transitions.
//! - `models`, `migration`, `config`, `caches`, and `utils` provide persistence,
//!   configuration loading, in-memory caching, and shared infrastructure.
//! - `kinds` collects shared enums and event types used across the crate.
//!
//! At runtime, the application typically starts in the TUI command, loads
//! configuration from disk, connects to the database, builds the root app in
//! `domains::tui`, and then coordinates keyboard input, timer ticks, and tab
//! rendering through the modules exposed here.
//!
//! The crate is split this way so the terminal UI, persistence logic, and CLI
//! entry flow remain decoupled enough to evolve independently while still
//! sharing the same domain types and helpers.

/// In-memory caches for frequently reused todo and session data.
pub mod caches;
/// CLI commands exposed by the application binary.
pub mod cmds;
/// Typed configuration loading and config section definitions.
pub mod config;
/// Top-level runtime domains such as the terminal app and seed logic.
pub mod domains;
/// Shared enums and event types used across the application.
pub mod kinds;
/// Database migrations for creating the application's tables.
pub mod migration;
/// Persistent domain models for todos and sessions.
pub mod models;
/// Runtime state containers backing the interactive tabs.
pub mod states;
/// Top-level TUI tabs and their shared trait.
pub mod tabs;
/// Shared helpers for paths, dates, storage, logging, and database access.
pub mod utils;
/// Reusable TUI widgets used by tabs and overlays.
pub mod widgets;
/// Background workers for terminal input and timer processing.
pub mod workers;
