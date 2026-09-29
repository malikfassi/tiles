use cosmwasm_std::Addr;
use tiles::core::tile::metadata::{PixelData, TileMetadata};
use tiles::core::tile::Tile;
use tiles::defaults::constants::DEFAULT_COLOR;

#[test]
fn test_default_tile_metadata() {
    let metadata = TileMetadata::default();
    assert_eq!(metadata.pixels.len(), 100, "a tile is a 10x10 grid");

    for (index, pixel) in metadata.pixels.iter().enumerate() {
        assert_eq!(pixel.id, index as u8, "pixels are numbered in order");
        assert_eq!(pixel.color, DEFAULT_COLOR, "blank pixels are white");
        assert_eq!(pixel.lease_expires_at, 0, "a blank pixel has no lease");
        assert!(pixel.leased_by.is_none(), "a blank pixel has no holder");
        assert_eq!(pixel.last_updated_at, 0, "a blank pixel was never written");
    }
}

#[test]
fn test_pixel_lease_helpers() {
    let pixel = PixelData {
        id: 7,
        color: "#FF0000".to_string(),
        lease_expires_at: 1000,
        leased_by: Some(Addr::unchecked("buyer")),
        last_updated_at: 500,
    };

    assert!(
        pixel.has_active_lease(999),
        "still leased just before expiry"
    );
    assert!(
        !pixel.has_active_lease(1000),
        "the lease ends at its timestamp"
    );
    assert!(!pixel.has_active_lease(1001), "expired lease");
    assert!(
        pixel.is_leased_by(&Addr::unchecked("buyer")),
        "the holder is recognised"
    );
    assert!(
        !pixel.is_leased_by(&Addr::unchecked("someone_else")),
        "another address is not the holder"
    );
}

#[test]
fn test_apply_update_writes_colour_and_lease() {
    let mut metadata = TileMetadata::default();
    let sender = Addr::unchecked("colourer");

    let update = tiles::core::tile::metadata::PixelUpdate {
        id: 42,
        color: "#0000FF".to_string(),
        expiration_duration: 3600,
    };

    metadata.apply_update(&update, &sender, 1000);

    let pixel = &metadata.pixels[42];
    assert_eq!(pixel.color, "#0000FF");
    assert_eq!(pixel.lease_expires_at, 4600, "now + duration");
    assert_eq!(pixel.leased_by.as_ref(), Some(&sender));
    assert_eq!(pixel.last_updated_at, 1000);

    // Other pixels are untouched.
    assert_eq!(metadata.pixels[41].color, DEFAULT_COLOR);
    assert!(metadata.pixels[41].leased_by.is_none());
}

/// The hash is a canonical digest: identical states hash alike, and any change
/// to any pixel changes the digest.
#[test]
fn test_hash_is_canonical_and_deterministic() {
    let metadata = TileMetadata::default();
    let tile_a = Tile {
        tile_hash: metadata.hash(),
        metadata: metadata.clone(),
    };
    let tile_b = Tile {
        tile_hash: TileMetadata::default().hash(),
        metadata: TileMetadata::default(),
    };

    assert!(!tile_a.tile_hash.is_empty());
    assert_eq!(tile_a.tile_hash, tile_b.tile_hash, "same state, same hash");

    let mut changed = metadata.clone();
    changed.pixels[0].color = "#FF0000".to_string();
    assert_ne!(
        changed.hash(),
        metadata.hash(),
        "a colour change must change the hash"
    );

    let mut leased = metadata.clone();
    leased.pixels[99].leased_by = Some(Addr::unchecked("someone"));
    leased.pixels[99].lease_expires_at = 7200;
    assert_ne!(
        leased.hash(),
        metadata.hash(),
        "a lease change must change the hash"
    );
}

#[test]
fn test_hash_does_not_depend_on_write_order() {
    let mut first = TileMetadata::default();
    let mut second = TileMetadata::default();
    let sender = Addr::unchecked("colourer");

    let red = tiles::core::tile::metadata::PixelUpdate {
        id: 0,
        color: "#FF0000".to_string(),
        expiration_duration: 3600,
    };
    let green = tiles::core::tile::metadata::PixelUpdate {
        id: 1,
        color: "#00FF00".to_string(),
        expiration_duration: 7200,
    };

    let updates = [red, green];
    for update in &updates {
        first.apply_update(update, &sender, 1000);
    }
    for update in updates.iter().rev() {
        second.apply_update(update, &sender, 1000);
    }

    assert_eq!(
        first.hash(),
        second.hash(),
        "the final state is what matters, not the write order"
    );
}
