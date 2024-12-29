from typing import List, Dict
import colorsys
import os

# Modern palette inspired by the given colors
MODERN_PALETTE = [
    '#9b87f5',  # Soft Purple
    '#7E69AB',  # Medium Purple
    '#6E59A5',  # Deep Purple
    '#D6BCFA',  # Light Purple
    '#FF719A',  # Pink
    '#FFA99F',  # Coral
    '#FFE29F',  # Light Yellow
    '#abecd6',  # Mint Green
    '#8B5CF6',  # Bright Purple
    '#EC4899',  # Hot Pink
    '#F472B6',  # Light Pink
    '#34D399',  # Emerald
    '#A78BFA',  # Lavender
    '#93C5FD',  # Sky Blue
    '#C4B5FD',  # Pale Purple
    '#6EE7B7',  # Seafoam
]

# Basic 16-color palette inspired by classic pixel art
BASIC_PALETTE = [
    '#000000',  # Black
    '#FFFFFF',  # White
    '#FF0000',  # Red
    '#00FF00',  # Green
    '#0000FF',  # Blue
    '#FFFF00',  # Yellow
    '#FF00FF',  # Magenta
    '#00FFFF',  # Cyan
    '#FF8800',  # Orange
    '#88FF00',  # Lime
    '#0088FF',  # Sky Blue
    '#8800FF',  # Purple
    '#FF0088',  # Pink
    '#008888',  # Teal
    '#888800',  # Olive
    '#880088',  # Violet
]

# Earthy/Natural palette (more muted and pleasant)
NATURAL_PALETTE = [
    '#1A1A1A',  # Almost Black
    '#F2F2F2',  # Almost White
    '#D4A373',  # Tan
    '#CCD5AE',  # Sage
    '#E9EDC9',  # Light Sage
    '#FEFAE0',  # Cream
    '#FAEDCD',  # Beige
    '#D4A373',  # Light Brown
    '#A98467',  # Medium Brown
    '#6C584C',  # Dark Brown
    '#ADC178',  # Olive Green
    '#A98467',  # Taupe
    '#DDA15E',  # Light Orange
    '#BC6C25',  # Dark Orange
    '#606C38',  # Forest Green
    '#283618',  # Dark Green
]

# Pastel palette (soft and pleasant)
PASTEL_PALETTE = [
    '#264653',  # Dark Blue
    '#F4F1DE',  # Cream
    '#E9C46A',  # Yellow
    '#F4A261',  # Orange
    '#E76F51',  # Red
    '#2A9D8F',  # Teal
    '#FFB5A7',  # Pink
    '#FCD5CE',  # Light Pink
    '#F8EDEB',  # White Pink
    '#F9DCC4',  # Peach
    '#FEC89A',  # Light Orange
    '#B7B7A4',  # Gray
    '#A5A58D',  # Olive
    '#6B705C',  # Dark Gray
    '#3D405B',  # Navy
    '#81B29A',  # Sage
]

# Vibrant palette (bold and energetic)
VIBRANT_PALETTE = [
    '#2D00F7',  # Electric Blue
    '#F20089',  # Hot Pink
    '#FF0000',  # Red
    '#00FF00',  # Green
    '#6A00F4',  # Purple
    '#8900F2',  # Violet
    '#A100F2',  # Indigo
    '#B100E8',  # Dark Pink
    '#BC00DD',  # Magenta
    '#D100D1',  # Bright Pink
    '#DB00B6',  # Deep Pink
    '#E500A4',  # Pink
    '#F20089',  # Light Pink
    '#FF0F7B',  # Coral
    '#FF3366',  # Red Pink
    '#FF6B6B',  # Light Red
]

def generate_gradient_palette(base_colors: List[str], steps: int = 16) -> List[str]:
    """Generate a gradient palette from base colors"""
    if len(base_colors) < 2:
        return base_colors
    
    palette = []
    for i in range(len(base_colors) - 1):
        color1 = base_colors[i]
        color2 = base_colors[i + 1]
        
        # Convert hex to RGB
        r1 = int(color1[1:3], 16)
        g1 = int(color1[3:5], 16)
        b1 = int(color1[5:7], 16)
        r2 = int(color2[1:3], 16)
        g2 = int(color2[3:5], 16)
        b2 = int(color2[5:7], 16)
        
        # Generate steps
        step_count = steps // (len(base_colors) - 1)
        for step in range(step_count):
            t = step / step_count
            r = int(r1 + (r2 - r1) * t)
            g = int(g1 + (g2 - g1) * t)
            b = int(b1 + (b2 - b1) * t)
            palette.append(f'#{r:02x}{g:02x}{b:02x}')
    
    return palette[:steps]  # Ensure we return exactly the requested number of colors

# Example gradient palette
GRADIENT_BASE = ['#FF0000', '#00FF00', '#0000FF', '#FF0000']  # Red -> Green -> Blue -> Red
GRADIENT_PALETTE = generate_gradient_palette(GRADIENT_BASE)

# All available palettes
PALETTES = {
    'modern': MODERN_PALETTE,  # Add modern palette first as default
    'basic': BASIC_PALETTE,
    'natural': NATURAL_PALETTE,
    'pastel': PASTEL_PALETTE,
    'vibrant': VIBRANT_PALETTE,
    'gradient': GRADIENT_PALETTE,
}

def get_palette(name: str) -> List[str]:
    """Get a palette by name"""
    return PALETTES.get(name, BASIC_PALETTE)

if __name__ == '__main__':
    # Generate a test image showing all palettes
    from PIL import Image, ImageDraw
    
    # Create image
    palette_height = 50
    margin = 10
    img_width = 800
    img_height = (palette_height + margin) * len(PALETTES)
    
    img = Image.new('RGB', (img_width, img_height), 'white')
    draw = ImageDraw.Draw(img)
    
    # Draw each palette
    for i, (name, palette) in enumerate(PALETTES.items()):
        y = i * (palette_height + margin)
        color_width = img_width // len(palette)
        
        # Draw color squares
        for j, color in enumerate(palette):
            x = j * color_width
            draw.rectangle(
                [x, y, x + color_width, y + palette_height],
                fill=color
            )
        
        # Draw palette name
        draw.text(
            (10, y + palette_height - 20),
            name,
            fill='black'
        )
    
    # Save image
    os.makedirs('scripts/assets/images', exist_ok=True)
    img.save('scripts/assets/images/palettes.png', 'PNG', optimize=True)
    print("Generated palette preview at scripts/assets/images/palettes.png") 