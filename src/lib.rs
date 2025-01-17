//! A pure-Rust client library to work with systemd.
//!
//! It provides support to interact with systemd components available
//! on modern Linux systems. This crate is entirely implemented
//! in Rust, and does not require an external libsystemd dynamic library.
//!
//! ```rust
//! use libsystemd::daemon::{self, NotifyState};
//!
//! fn notify_ready() -> bool {
//!     if !daemon::booted() {
//!         println!("Not running systemd, early exit.");
//!         return false;
//!     };
//!
//!     let sent = daemon::notify(true, &[NotifyState::Ready]).expect("notify failed");
//!     if !sent {
//!         println!("Notification not sent!");
//!     };
//!     sent
//! }
//! ```

#[macro_use]
extern crate error_chain;
extern crate hmac;
extern crate libc;
extern crate nix;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sha2;
#[macro_use]
extern crate try_or;
extern crate uuid;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

/// Interfaces for systemd-aware daemons.
pub mod daemon;

/// Error handling.
pub mod errors;

/// Helpers for working with systemd units.
pub mod unit;

/// APIs for processing 128-bits IDs.
pub mod id128;
