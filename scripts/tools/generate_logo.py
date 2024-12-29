from PIL import Image, ImageDraw
import os

def generate_logo(size=512):
    # Create a new image with white background
    img = Image.new('RGB', (size, size), 'white')
    draw = ImageDraw.Draw(img)
    
    # Calculate grid size
    grid_size = 8  # 8x8 grid for the logo
    cell_size = size // grid_size
    
    # Draw a sample pixel art pattern
    colors = ['#FF0000', '#00FF00', '#0000FF', '#FFFF00']
    for y in range(grid_size):
        for x in range(grid_size):
            if (x + y) % 2 == 0:  # Create a checkered pattern
                color = colors[(x + y) % len(colors)]
                draw.rectangle(
                    [x * cell_size, y * cell_size, 
                     (x + 1) * cell_size, (y + 1) * cell_size],
                    fill=color
                )
    
    # Add grid lines
    for i in range(grid_size + 1):
        line_pos = i * cell_size
        draw.line([(line_pos, 0), (line_pos, size)], fill='black', width=2)
        draw.line([(0, line_pos), (size, line_pos)], fill='black', width=2)
    
    # Create output directory if it doesn't exist
    os.makedirs('scripts/assets/images', exist_ok=True)
    
    # Save the image
    img.save('scripts/assets/images/logo.png', 'PNG', optimize=True)

if __name__ == '__main__':
    generate_logo() 