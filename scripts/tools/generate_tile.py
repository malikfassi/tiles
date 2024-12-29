from PIL import Image, ImageDraw, ImageFont
import json
import os
from typing import List, Dict

def generate_tile_image(
    token_id: int,
    pixels: List[Dict],
    size: int = 500,
    grid_size: int = 10
) -> Image.Image:
    # Create a new image with white background
    img = Image.new('RGB', (size, size), 'white')
    draw = ImageDraw.Draw(img)
    
    # Calculate cell size
    cell_size = size // grid_size
    
    # Draw pixels
    for pixel in pixels:
        x = (pixel['id'] % grid_size) * cell_size
        y = (pixel['id'] // grid_size) * cell_size
        color = pixel['color']
        
        draw.rectangle(
            [x, y, x + cell_size, y + cell_size],
            fill=color
        )
    
    # Draw grid lines
    for i in range(grid_size + 1):
        line_pos = i * cell_size
        # Vertical line
        draw.line([(line_pos, 0), (line_pos, size)], fill='black', width=1)
        # Horizontal line
        draw.line([(0, line_pos), (size, line_pos)], fill='black', width=1)
    
    # Add coordinates
    font_size = cell_size // 4
    try:
        font = ImageFont.truetype("Arial", font_size)
    except:
        font = ImageFont.load_default()
    
    # Add coordinates in corners of cells
    for y in range(grid_size):
        for x in range(grid_size):
            pixel_id = y * grid_size + x
            coord_text = str(pixel_id)
            # Draw coordinate with small offset from corner
            draw.text(
                (x * cell_size + 2, y * cell_size + 2),
                coord_text,
                fill='black',
                font=font
            )
    
    return img

def save_tile_image(token_id: int, metadata_file: str):
    # Read metadata
    with open(metadata_file, 'r') as f:
        metadata = json.load(f)
    
    # Generate image
    img = generate_tile_image(token_id, metadata['pixels'])
    
    # Create output directory if it doesn't exist
    os.makedirs('scripts/assets/images', exist_ok=True)
    
    # Save the image
    output_path = f'scripts/assets/images/{token_id}.png'
    img.save(output_path, 'PNG', optimize=True)
    return output_path

if __name__ == '__main__':
    # Example usage
    metadata = {
        'pixels': [
            {'id': i, 'color': '#FFFFFF'} 
            for i in range(100)
        ]
    }
    
    # Save example metadata
    os.makedirs('scripts/assets/metadata', exist_ok=True)
    with open('scripts/assets/metadata/example.json', 'w') as f:
        json.dump(metadata, f, indent=2)
    
    # Generate example tile
    save_tile_image(1, 'scripts/assets/metadata/example.json') 