import json
import os
import random
from typing import List, Dict
from colors import get_palette, PALETTES

def create_checkerboard_pattern(palette_name: str = 'basic') -> List[Dict]:
    """Create a checkerboard pattern with alternating colors"""
    palette = get_palette(palette_name)
    # Use first dark and first light colors from palette
    dark_colors = [c for c in palette[:8]]  # First half of palette
    light_colors = [c for c in palette[8:]]  # Second half of palette
    
    pixels = []
    for i in range(100):
        x = i % 10
        y = i // 10
        # Alternate between dark and light colors
        colors = dark_colors if (x + y) % 2 == 0 else light_colors
        color = random.choice(colors)  # Random color from the selected group
        pixels.append({
            'id': i,
            'color': color
        })
    return pixels

def create_modern_wave_pattern(palette_name: str = 'modern') -> List[Dict]:
    """Create a wave-like pattern using modern colors"""
    palette = get_palette(palette_name)
    pixels = []
    
    for i in range(100):
        x = i % 10
        y = i // 10
        # Create a wave pattern based on position
        wave = (x + y + int(2 * (x * y / 10))) % len(palette)
        color = palette[wave]
        pixels.append({
            'id': i,
            'color': color
        })
    return pixels

def create_gradient_pattern(palette_name: str = 'gradient') -> List[Dict]:
    """Create a gradient pattern"""
    palette = get_palette(palette_name)
    pixels = []
    
    for i in range(100):
        x = i % 10
        y = i // 10
        # Use position to index into palette
        color_index = ((x + y) * len(palette)) // 20
        color_index = min(color_index, len(palette) - 1)
        color = palette[color_index]
        
        pixels.append({
            'id': i,
            'color': color
        })
    return pixels

def create_random_pattern(palette_name: str = 'vibrant') -> List[Dict]:
    """Create a random pattern with some clustering"""
    palette = get_palette(palette_name)
    pixels = []
    
    for i in range(100):
        x = i % 10
        y = i // 10
        # 70% chance to use a color similar to neighbors
        if random.random() < 0.7 and i > 0:
            color = pixels[i-1]['color']  # Use previous pixel's color
        else:
            color = random.choice(palette)
        pixels.append({
            'id': i,
            'color': color
        })
    return pixels

def create_spiral_pattern(palette_name: str = 'pastel') -> List[Dict]:
    """Create a spiral pattern"""
    palette = get_palette(palette_name)
    pixels = []
    
    # Initialize with background color
    for i in range(100):
        pixels.append({
            'id': i,
            'color': palette[1]  # Usually a light color
        })
    
    # Create spiral
    directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]  # right, down, left, up
    x, y = 0, 0
    dir_idx = 0
    steps = 10
    step_count = 0
    color_idx = 0
    
    for _ in range(100):
        if 0 <= x < 10 and 0 <= y < 10:
            pixel_id = y * 10 + x
            pixels[pixel_id]['color'] = palette[color_idx % len(palette)]
        
        step_count += 1
        if step_count == steps:
            dir_idx = (dir_idx + 1) % 4
            if dir_idx % 2 == 0:
                steps -= 1
            step_count = 0
            color_idx += 1
        
        dx, dy = directions[dir_idx]
        x += dx
        y += dy
    
    return pixels

def generate_test_data():
    # Create test patterns with different palettes
    patterns = {
        'modern_wave': (create_modern_wave_pattern, 'modern'),
        'modern_gradient': (create_gradient_pattern, 'modern'),
        'modern_random': (create_random_pattern, 'modern'),
        'modern_spiral': (create_spiral_pattern, 'modern'),
        'modern_checker': (create_checkerboard_pattern, 'modern'),
        'checkerboard_basic': (create_checkerboard_pattern, 'basic'),
        'gradient_pastel': (create_gradient_pattern, 'pastel'),
        'gradient_vibrant': (create_gradient_pattern, 'vibrant'),
        'random_vibrant': (create_random_pattern, 'vibrant'),
        'random_natural': (create_random_pattern, 'natural'),
        'spiral_pastel': (create_spiral_pattern, 'pastel'),
    }
    
    # Create output directory
    os.makedirs('scripts/assets/metadata', exist_ok=True)
    os.makedirs('scripts/assets/images', exist_ok=True)
    
    # Save each pattern and generate image
    for pattern_name, (pattern_func, palette_name) in patterns.items():
        print(f"Generating {pattern_name}...")
        # Save metadata
        pixels = pattern_func(palette_name)
        metadata = {'pixels': pixels}
        metadata_file = f'scripts/assets/metadata/{pattern_name}.json'
        with open(metadata_file, 'w') as f:
            json.dump(metadata, f, indent=2)
        
        # Generate image
        from generate_tile import save_tile_image
        save_tile_image(pattern_name, metadata_file)
        # Rename the file to match pattern name
        os.rename(
            f'scripts/assets/images/{pattern_name}.png',
            f'scripts/assets/images/tile_{pattern_name}.png'
        )

if __name__ == '__main__':
    generate_test_data()
    print("Generated test patterns with different palettes")
    print("Check scripts/assets/images/ for the generated images") 