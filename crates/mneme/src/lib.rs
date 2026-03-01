//! aletheia-mneme — session store and memory engine
//!
//! Mneme (Μνήμη) — "memory." Manages sessions, messages, and usage tracking
//! via embedded `SQLite` (`rusqlite`). Future: `CozoDB` for vectors + graph.
//!
//! Depends on `aletheia-koina` for types and errors.

pub mod error;
pub mod schema;
pub mod store;
pub mod types;
