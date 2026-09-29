use crate::defaults::constants::{DEFAULT_COLOR, PIXELS_PER_TILE};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};
use sha2::{Digest, Sha256};

/// A single pixel of a tile.
///
/// `lease_expires_at` is the block time until which the colour is protected: while it
/// is in the future, only `leased_by` may write this pixel (ADR 0004).
#[cw_serde]
pub struct PixelData {
    pub id: u8,
    pub color: String,
    /// Block time at which the lease ends. The epoch means the pixel was never painted.
    pub lease_expires_at: Timestamp,
    /// Address that holds the lease. `None` when the pixel has never been painted.
    pub leased_by: Option<Addr>,
    /// Block time of the last write. The epoch means it was never written.
    pub last_updated_at: Timestamp,
}

impl Default for PixelData {
    fn default() -> Self {
        Self {
            id: 0,
            color: DEFAULT_COLOR.to_string(),
            lease_expires_at: Timestamp::default(),
            leased_by: None,
            last_updated_at: Timestamp::default(),
        }
    }
}

impl PixelData {
    /// Whether the pixel is currently protected by a lease.
    pub fn has_active_lease(&self, now: Timestamp) -> bool {
        self.lease_expires_at > now
    }

    /// Whether `sender` holds the active lease on this pixel.
    pub fn is_leased_by(&self, sender: &Addr) -> bool {
        self.leased_by.as_ref() == Some(sender)
    }
}

/// The 100 pixels of a tile.
///
/// Stored in the NFT extension, so it travels with the token. Held as a `Vec` because
/// serde does not implement `Serialize` for arrays longer than 32 elements, but the
/// length is enforced to `PIXELS_PER_TILE` at construction time.
#[cw_serde]
pub struct TileMetadata {
    pub pixels: Vec<PixelData>,
}

impl Default for TileMetadata {
    fn default() -> Self {
        Self {
            pixels: (0..PIXELS_PER_TILE)
                .map(|i| PixelData {
                    id: i as u8,
                    ..PixelData::default()
                })
                .collect(),
        }
    }
}

impl TileMetadata {
    /// Applies one update. Validation is the caller's responsibility.
    pub fn apply_update(&mut self, update: &PixelUpdate, sender: &Addr, now: Timestamp) {
        let pixel = &mut self.pixels[update.id as usize];
        pixel.color = update.color.clone();
        pixel.lease_expires_at = update.expiration_timestamp(now);
        pixel.leased_by = Some(sender.clone());
        pixel.last_updated_at = now;
    }

    /// Number of pixels currently under an active lease.
    pub fn leased_pixel_count(&self, now: Timestamp) -> usize {
        self.pixels
            .iter()
            .filter(|pixel| pixel.has_active_lease(now))
            .count()
    }

    /// Canonical hash of the tile state.
    ///
    /// Serialises with serde rather than string concatenation, so that two different
    /// states can never produce the same digest.
    pub fn hash(&self) -> String {
        let bytes = serde_json::to_vec(&self.pixels).expect("tile metadata is always serialisable");
        let digest = Sha256::digest(bytes);
        hex_encode(&digest)
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

/// A requested write on a pixel: the new colour and how long the lease should last.
#[cw_serde]
pub struct PixelUpdate {
    pub id: u8,
    pub color: String,
    /// Lease duration in seconds.
    pub expiration_duration: u64,
}

impl PixelUpdate {
    /// Block time at which the lease bought by this update would end.
    pub fn expiration_timestamp(&self, now: Timestamp) -> Timestamp {
        now.plus_seconds(self.expiration_duration)
    }
}
