use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

use anyhow::anyhow;
use gpui::{Image, ImageFormat};
use itertools::Itertools;
use nostr_sdk::prelude::*;
use qrcode_generator::QrCodeEcc;

pub mod debounced_delay;
pub mod display;
pub mod handle_auth;
pub mod nip05;
pub mod nip96;

pub fn room_hash(event: &Event) -> u64 {
    let mut hasher = DefaultHasher::new();
    let mut pubkeys: Vec<PublicKey> = vec![];

    // Add all public keys from event
    pubkeys.push(event.pubkey);
    pubkeys.extend(event.tags.public_keys().collect::<Vec<_>>());

    // Generate unique hash
    pubkeys
        .into_iter()
        .unique()
        .sorted()
        .collect::<Vec<_>>()
        .hash(&mut hasher);

    hasher.finish()
}

pub fn parse_pubkey_from_str(content: &str) -> Result<PublicKey, anyhow::Error> {
    if content.starts_with("nprofile1") {
        Ok(Nip19Profile::from_bech32(content)?.public_key)
    } else if content.starts_with("npub1") {
        Ok(PublicKey::parse(content)?)
    } else {
        Err(anyhow!("Invalid public key"))
    }
}

pub fn string_to_qr(data: &str) -> Option<Arc<Image>> {
    let Ok(bytes) = qrcode_generator::to_png_to_vec_from_str(data, QrCodeEcc::Medium, 256) else {
        return None;
    };

    Some(Arc::new(Image::from_bytes(ImageFormat::Png, bytes)))
}

pub fn compare<T>(a: &[T], b: &[T]) -> bool
where
    T: Eq + Hash,
{
    let a: HashSet<_> = a.iter().collect();
    let b: HashSet<_> = b.iter().collect();

    a == b
}
