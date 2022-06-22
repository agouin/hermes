// TODO: disable unwraps:
// https://github.com/informalsystems/ibc-rs/issues/987
// #![cfg_attr(not(test), deny(clippy::unwrap_used))]

#![no_std]
#![allow(clippy::large_enum_variant)]
#![deny(
    warnings,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the InterBlockchain Communication (IBC) protocol in Rust. IBC is
//! a distributed protocol that enables communication between distinct sovereign blockchains.
//! Loose analogies may be drawn between the IBC protocol and the TCP/UDP protocols that enable
//! communication over the internet via packet streaming. Indeed, IBC also encodes the notion of
//! ordered and unordered packet streams.
//!
//! The layout of this crate mirrors the classification of the [Interchain
//! Standards][ics-standards]. The classification consists of [Core][core], [Clients][clients],
//! [Applications][applications], and [Relayer][relayer].
//!
//! `Core` consists of the designs and logic pertaining to the transport, authentication, and
//! ordering layers of the IBC protocol, the fundamental pieces.
//!
//! `Clients` consists of implementations of client verification algorithms (following the base
//! client interface that is defined in `Core`) for specific types of chains. A chain uses these
//! verification algorithms to verify the state of remote chains.
//!
//! `Applications` consists of various packet encoding and processing semantics which underpin the
//! various types of transactions that users can perform on any IBC-compliant chain.
//!
//! `Relayer` contains utilities for testing the `ibc` crate against the [Hermes IBC relayer][relayer-repo]. It acts
//! as scaffolding for gluing the `ibc` crate with Hermes for testing purposes.
//!
//! [core]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/core
//! [clients]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/clients
//! [applications]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/applications
//! [ics-standards]: https://github.com/cosmos/ibc#interchain-standards
//! [relayer]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/relayer
//! [relayer-repo]: https://github.com/informalsystems/ibc-rs/tree/master/relayer

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod prelude;

pub mod bigint;
pub mod ics02_client;
pub mod ics03_connection;
pub mod ics04_channel;
pub mod ics05_port;
pub mod ics23_commitment;
pub mod ics24_host;
pub mod proofs;
pub mod signer;
pub mod timestamp;

pub mod serializers;

/// Re-export of ICS 002 Height domain type
pub type Height = ics02_client::height::Height;