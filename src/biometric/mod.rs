//! Biometric binding simulation for SSI Nigeria identity cards.
//!
//! In production, this logic runs inside a Hardware Secure Element (HSE)
//! chip embedded in the physical identity card. The private key never leaves
//! the chip; biometric matching and signing happen on-device.

pub mod card;
