#!/usr/bin/env python3

import os
import json
import random
from datetime import datetime, timedelta
from pathlib import Path
from PIL import Image, ImageDraw
from colors import MODERN_PALETTE

# Constants from messages/constants.json
with open('scripts/messages/constants.json', 'r') as f:
    constants = json.load(f)
    
DEFAULT_COLOR = constants['DEFAULT_COLOR']
TILE_SIZE = constants['TILE_SIZE']
PIXELS_PER_TILE = constants['PIXELS_PER_TILE']
PIXEL_MIN_EXPIRATION = constants['PIXEL_MIN_EXPIRATION']
PIXEL_MAX_EXPIRATION = constants['PIXEL_MAX_EXPIRATION']
COLLECTION_NAME = constants['COLLECTION_NAME']
COLLECTION_DESCRIPTION = constants['COLLECTION_DESCRIPTION']
MAX_TOKEN_LIMIT = constants['MAX_TOKEN_LIMIT']

# Sample Stargaze addresses for randomization
SAMPLE_ADDRESSES = [
    "stars1ve46fjrhcrum94c7d8yc2wsdz8cpll7xk2ncd5",
    "stars1tdwc4y36dup0qj8qm8x3dj5kx8nyssxwk9hfrc",
    "stars1pr3mhj7yqf9wjc8vxzk0qvyqz8ue7mfp5e7852",
    "stars1cad0efr25farisqj9g3kmhfj74d5yfxtkx7v82",
    "stars1m5dncvfv7lvpjndv9g6c7pu5k4ufr4pdg8pf28"
]

def get_random_timestamp(min_age: int = 0, max_age: int = 86400) -> int:
    """Get a random timestamp between now-max_age and now-min_age."""
    now = datetime.now()
    age = random.randint(min_age, max_age)
    past_time = now - timedelta(seconds=age)
    return int(past_time.timestamp())

def get_random_expiration(current_time: int) -> int:
    """Get a random expiration timestamp between min and max expiration from current_time."""
    duration = random.randint(PIXEL_MIN_EXPIRATION, PIXEL_MAX_EXPIRATION)
    return current_time + duration

def create_tile_image(tile_id: int, size: tuple = (500, 500)) -> Image:
    """Create a tile image with random colored pixels."""
    background_color = DEFAULT_COLOR
    cell_size = size[0] // TILE_SIZE
    
    # Create base image
    img = Image.new('RGB', size, background_color)
    draw = ImageDraw.Draw(img)
    
    # Create a list of all possible positions
    positions = [(x, y) for x in range(TILE_SIZE) for y in range(TILE_SIZE)]
    
    # Randomly select 30-50% of positions to color
    num_colored = random.randint(30, 50)
    colored_positions = random.sample(positions, num_colored)
    
    # Draw colored cells
    pixel_colors = {}  # Store colors for metadata
    for x, y in colored_positions:
        color = random.choice(MODERN_PALETTE)
        pixel_colors[(x, y)] = color
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
    
    return img, pixel_colors

def create_metadata(tile_id: int, pixel_colors: dict) -> dict:
    """Create metadata for a tile."""
    current_time = get_random_timestamp(0, 86400)  # Random time in last 24 hours
    pixels = []
    active_colors = set()
    pixel_attributes = []
    
    for i in range(PIXELS_PER_TILE):
        x = i % TILE_SIZE
        y = i // TILE_SIZE
        
        if (x, y) in pixel_colors:
            color = pixel_colors[(x, y)]
            active_colors.add(color)
            # Colored pixels have random recent timestamps
            last_updated_at = get_random_timestamp(0, 43200)  # Last 12 hours
        else:
            color = DEFAULT_COLOR
            # White pixels have older timestamps
            last_updated_at = get_random_timestamp(43200, 86400)  # 12-24 hours ago
        
        expiration_timestamp = get_random_expiration(last_updated_at)
        last_updated_by = random.choice(SAMPLE_ADDRESSES)
        
        # Add pixel data as attributes
        pixel_attributes.extend([
            {
                "trait_type": f"Pixel {x},{y} Color",
                "value": color
            },
            {
                "trait_type": f"Pixel {x},{y} Expiration",
                "value": datetime.fromtimestamp(expiration_timestamp).strftime("%Y-%m-%d %H:%M:%S")
            },
            {
                "trait_type": f"Pixel {x},{y} Last Updated By",
                "value": last_updated_by
            }
        ])
        
        # Keep pixel data for contract use
        pixels.append({
            "id": i,
            "color": color,
            "expiration_timestamp": expiration_timestamp,
            "last_updated_by": last_updated_by,
            "last_updated_at": last_updated_at
        })
    
    return {
        "name": f"{COLLECTION_NAME} #{tile_id}",
        "description": COLLECTION_DESCRIPTION,
        # Include full IPFS URL format for image
        "image": f"ipfs://IMAGES_CID/{tile_id}.png",  # IMAGES_CID will be replaced after upload
        "external_url": f"https://app.stargaze.zone/tokens/{tile_id}",
        "attributes": [
            {
                "trait_type": "Grid Size",
                "value": f"{TILE_SIZE}x{TILE_SIZE}"
            },
            {
                "trait_type": "Total Pixels",
                "value": str(PIXELS_PER_TILE)
            },
            {
                "trait_type": "Active Colors",
                "value": str(len(active_colors))
            },
            {
                "trait_type": "Color Palette",
                "value": "Modern"
            },
            {
                "trait_type": "Colored Pixels",
                "value": str(len(pixel_colors))
            }
        ],
        "contract_data": {  # Keep contract-specific data separate
            "pixels": pixels
        }
    }

def update_metadata_image_urls(images_cid: str):
    """Update all metadata files with the correct image CIDs."""
    metadata_dir = Path("ipfs/metadata")
    for metadata_file in sorted(metadata_dir.glob("*.json"), key=lambda x: int(x.stem)):
        with open(metadata_file) as f:
            metadata = json.load(f)
        
        # Update image URL with actual CID
        metadata["image"] = metadata["image"].replace("IMAGES_CID", images_cid)
        
        with open(metadata_file, "w") as f:
            json.dump(metadata, f, indent=2)
    print(f"✅ Updated all metadata files with images CID: {images_cid}")

def main():
    # Create directories
    base_dir = Path("ipfs")
    images_dir = base_dir / "images"
    metadata_dir = base_dir / "metadata"
    
    # Clean up existing directories
    import shutil
    if base_dir.exists():
        shutil.rmtree(base_dir)
    
    # Create fresh directories
    for dir in [base_dir, images_dir, metadata_dir]:
        dir.mkdir(parents=True, exist_ok=True)
    
    # Generate logo first
    logo, _ = create_tile_image(0)  # Use 0 as seed for logo
    logo.save(base_dir / "logo.png")
    print("✅ Logo generated")
    
    # Generate tiles
    num_tiles = MAX_TOKEN_LIMIT
    for tile_id in range(1, num_tiles + 1):
        # Generate and save image
        img, pixel_colors = create_tile_image(tile_id)
        img.save(images_dir / f"{tile_id}.png")
        
        # Create metadata
        metadata = create_metadata(tile_id, pixel_colors)
        with open(metadata_dir / f"{tile_id}.json", "w") as f:
            json.dump(metadata, f, indent=2)
        
        print(f"✅ Generated tile {tile_id}/{num_tiles}")
    
    print("\n✨ All tiles generated successfully!")
    print("\nDirectory structure created (Stargaze compatible):")
    print("ipfs/")
    print("├── images/")
    print("│   ├��─ 1.png")
    print("│   ├── 2.png")
    print("│   └── ...")
    print("├── metadata/")
    print("│   ├── 1.json")
    print("│   ├── 2.json")
    print("│   └── ...")
    print("└── logo.png")
    print("\nNext steps:")
    print("1. Upload images:")
    print("   pinata-web3 upload ipfs/images")
    print("2. Update metadata with images CID:")
    print("   python scripts/update_metadata.py <IMAGES_CID>")
    print("3. Upload metadata:")
    print("   pinata-web3 upload ipfs/metadata")

if __name__ == "__main__":
    main() 