#!/usr/bin/env python3

import os
import json
import random
from datetime import datetime, timedelta
from PIL import Image, ImageDraw
from colors import MODERN_PALETTE

# Constants from Rust code
PIXELS_PER_TILE = 100
TILE_SIZE = 10
DEFAULT_COLOR = "#FFFFFF"
PIXEL_MIN_EXPIRATION = 3600  # 1 hour
PIXEL_MAX_EXPIRATION = 86400  # 24 hours

def generate_modern_color():
    """Generate a modern color from the predefined palette."""
    return random.choice(MODERN_PALETTE)

def create_logo():
    """Create a modern logo with random colored cells."""
    size = (500, 500)
    background_color = "#FFFFFF"
    cell_size = size[0] // TILE_SIZE
    
    # Create base image
    img = Image.new('RGB', size, background_color)
    draw = ImageDraw.Draw(img)
    
    # Draw colored cells
    for x in range(TILE_SIZE):
        for y in range(TILE_SIZE):
            if random.random() > 0.7:  # 30% chance of colored cell
                color = generate_modern_color()
                draw.rectangle(
                    [x * cell_size, y * cell_size, (x + 1) * cell_size, (y + 1) * cell_size],
                    fill=color
                )
    
    # Draw grid lines
    grid_color = "#000000"
    for i in range(TILE_SIZE + 1):
        line_pos = i * cell_size
        draw.line([(line_pos, 0), (line_pos, size[1])], fill=grid_color, width=2)
        draw.line([(0, line_pos), (size[0], line_pos)], fill=grid_color, width=2)
    
    # Save the logo
    os.makedirs("ipfs/images", exist_ok=True)
    img.save("ipfs/images/logo.png")
    print("✅ Logo generated successfully")

def create_metadata_template():
    """Create a metadata template for NFTs with complete TileMetadata structure."""
    os.makedirs("ipfs/metadata", exist_ok=True)
    
    # Generate pixel data
    current_time = int(datetime.now().timestamp())
    pixels = []
    pixel_attributes = []
    active_colors = set()

    for i in range(PIXELS_PER_TILE):
        # Random expiration between min and max
        expiration_duration = random.randint(PIXEL_MIN_EXPIRATION, PIXEL_MAX_EXPIRATION)
        expiration_timestamp = current_time + expiration_duration
        color = generate_modern_color() if random.random() > 0.7 else DEFAULT_COLOR
        
        # Add pixel data
        pixels.append({
            "id": i,
            "color": color,
            "expiration_timestamp": expiration_timestamp,
            "last_updated_by": "stars1...",  # Placeholder address
            "last_updated_at": current_time
        })

        # Track active colors
        if color != DEFAULT_COLOR:
            active_colors.add(color)

        # Add pixel attributes
        x = i % TILE_SIZE
        y = i // TILE_SIZE
        pixel_attributes.extend([
            {
                "trait_type": f"Pixel {x},{y} Color",
                "value": color
            },
            {
                "trait_type": f"Pixel {x},{y} Expiration",
                "value": datetime.fromtimestamp(expiration_timestamp).strftime("%Y-%m-%d %H:%M:%S")
            }
        ])
    
    # Create metadata with TileMetadata structure
    metadata = {
        "name": "Tile #1",
        "description": "A collaborative pixel art canvas tile on Stargaze",
        "image": "ipfs://<CID>/images/1.png",
        "attributes": [
            # Add all pixel attributes
            *pixel_attributes
        ],
        "tile_metadata": {
            "pixels": pixels
        }
    }
    
    # Save template
    with open("ipfs/metadata/template.json", "w") as f:
        json.dump(metadata, f, indent=2)
    print("✅ Metadata template generated successfully")

def main():
    print("🎨 Generating IPFS content...")
    create_logo()
    create_metadata_template()
    print("\n✨ All content generated successfully!")
    print("\nNext steps:")
    print("1. Install IPFS CLI if not installed: brew install ipfs")
    print("2. Initialize IPFS if not initialized: ipfs init")
    print("3. Upload content: ipfs add -r ipfs/")
    print("4. Update constants.rs with the new CID")

if __name__ == "__main__":
    main() 