from PIL import Image, ImageDraw
import json
import os
import random
from typing import List, Dict
from colors import get_palette, PALETTES
from generate_test_data import (
    create_checkerboard_pattern,
    create_gradient_pattern,
    create_random_pattern,
    create_spiral_pattern,
    create_modern_wave_pattern
)

def generate_mixed_tile(palette_name: str) -> List[Dict]:
    """Generate a tile with a random pattern type"""
    pattern_funcs = [
        create_checkerboard_pattern,
        create_gradient_pattern,
        create_random_pattern,
        create_spiral_pattern,
        create_modern_wave_pattern
    ]
    chosen_pattern = random.choice(pattern_funcs)
    return chosen_pattern(palette_name)

def generate_canvas(
    canvas_width: int = 100,  # Number of tiles horizontally (100x100 grid)
    canvas_height: int = 100, # Number of tiles vertically
    tile_size: int = 100,    # Pixels per tile
    pattern_type: str = 'mixed',  # 'mixed' or 'random'
    palette_name: str = 'modern',
    scale: float = 0.1       # Scale factor for preview (0.1 = 10% size)
) -> Image.Image:
    """Generate a full canvas image by combining multiple tiles"""
    
    # Calculate full image size
    full_width = int(canvas_width * tile_size * scale)
    full_height = int(canvas_height * tile_size * scale)
    scaled_tile_size = int(tile_size * scale)
    
    print(f"Generating {canvas_width}x{canvas_height} canvas with {palette_name} palette...")
    print(f"Output size: {full_width}x{full_height} pixels")
    
    # Create a new image with white background
    canvas = Image.new('RGB', (full_width, full_height), 'white')
    draw = ImageDraw.Draw(canvas)
    
    # Generate tiles
    for y in range(canvas_height):
        if y % 10 == 0:
            print(f"Progress: {y}/{canvas_height} rows")
        for x in range(canvas_width):
            # Generate pattern for this tile
            if pattern_type == 'mixed':
                pixels = generate_mixed_tile(palette_name)
            else:  # random
                pixels = create_random_pattern(palette_name)
            
            # Draw each pixel in the tile
            for i, pixel in enumerate(pixels):
                px = (i % 10) * (scaled_tile_size // 10)
                py = (i // 10) * (scaled_tile_size // 10)
                # Offset by tile position
                px += x * scaled_tile_size
                py += y * scaled_tile_size
                
                # Draw the pixel
                draw.rectangle(
                    [px, py, 
                     px + max(1, scaled_tile_size // 10), 
                     py + max(1, scaled_tile_size // 10)],
                    fill=pixel['color']
                )
    
    # Draw grid lines (only major grid lines for readability)
    for x in range(0, canvas_width + 1, 10):
        line_x = x * scaled_tile_size
        draw.line([(line_x, 0), (line_x, full_height)], fill='black', width=2)
    
    for y in range(0, canvas_height + 1, 10):
        line_y = y * scaled_tile_size
        draw.line([(0, line_y), (full_width, line_y)], fill='black', width=2)
    
    return canvas

def save_canvas_variations():
    """Generate and save canvas images with different palettes and patterns"""
    variations = [
        ('modern', 'mixed'),
        ('modern', 'random'),
        ('pastel', 'mixed'),
        ('pastel', 'random'),
        ('vibrant', 'mixed'),
        ('vibrant', 'random'),
        ('natural', 'mixed'),
        ('natural', 'random'),
    ]
    
    # Create output directory
    os.makedirs('scripts/assets/images', exist_ok=True)
    
    # Generate each variation
    for palette_name, pattern_type in variations:
        print(f"\nGenerating {palette_name} palette with {pattern_type} pattern...")
        canvas = generate_canvas(
            canvas_width=100,
            canvas_height=100,
            pattern_type=pattern_type,
            palette_name=palette_name
        )
        output_path = f'scripts/assets/images/canvas_{palette_name}_{pattern_type}.png'
        canvas.save(output_path, 'PNG', optimize=True)
        print(f"Saved {output_path}")

if __name__ == '__main__':
    save_canvas_variations()
    print("\nAll canvas variations generated!")
    print("Check scripts/assets/images/ for the generated images") 