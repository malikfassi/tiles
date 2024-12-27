use cosmwasm_std::Addr;
use tiles::core::tile::metadata::{PixelData, TileMetadata};
use tiles::defaults::constants::DEFAULT_COLOR;

#[test]
fn test_default_tile_metadata() {
    let metadata = TileMetadata::default();
    assert_eq!(metadata.pixels.len(), 100); // 10x10 grid

    // Check that all pixels are initialized with default values
    for pixel in metadata.pixels.iter() {
        assert_eq!(pixel.color, DEFAULT_COLOR);
        assert_eq!(pixel.expiration_timestamp, 0);
        assert_eq!(pixel.last_updated_by, Addr::unchecked(""));
        assert_eq!(pixel.last_updated_at, 0);
        assert_eq!(pixel.id, 0u32);
    }
}

#[test]
fn test_pixel_update() {
    let mut metadata = TileMetadata::default();
    let pixel_id = 42;
    let color = "#FF0000".to_string();
    let expiration_timestamp = 3600;
    let last_updated_by = Addr::unchecked("user");
    let last_updated_at = 1000;

    // Update a pixel
    metadata.pixels[pixel_id] = PixelData {
        id: pixel_id as u32,
        color: color.clone(),
        expiration_timestamp,
        last_updated_by: last_updated_by.clone(),
        last_updated_at,
    };

    // Verify the update
    assert_eq!(metadata.pixels[pixel_id].color, color);
    assert_eq!(
        metadata.pixels[pixel_id].expiration_timestamp,
        expiration_timestamp
    );
    assert_eq!(metadata.pixels[pixel_id].last_updated_by, last_updated_by);
    assert_eq!(metadata.pixels[pixel_id].last_updated_at, last_updated_at);
    assert_eq!(metadata.pixels[pixel_id].id, pixel_id as u32);

    // Verify other pixels remain unchanged
    for (i, pixel) in metadata.pixels.iter().enumerate() {
        if i != pixel_id {
            assert_eq!(pixel.color, DEFAULT_COLOR);
            assert_eq!(pixel.expiration_timestamp, 0);
            assert_eq!(pixel.last_updated_by, Addr::unchecked(""));
            assert_eq!(pixel.last_updated_at, 0);
            assert_eq!(pixel.id, 0u32);
        }
    }
}

#[test]
fn test_pixel_bounds() {
    let metadata = TileMetadata::default();
    assert_eq!(metadata.pixels.len(), 100);
}
