from PIL import Image, ImageFont, ImageDraw, ImageFilter, ImageChops
import math

IMAGE_SIZE = 128
COLUMNS = 10
ROWS = 7
FILTER_SIZE = 2

FONT_SIZE = math.floor(IMAGE_SIZE / ROWS) - FILTER_SIZE


CHARS = """
yz:-<>![] \
opqrstuvwx
efghijklmn
UVWXYZabcd
KLMNOPQRST
ABCDFEGHIJ
0123456789
""".replace("\n", "")
print(len(CHARS))


FONT = ImageFont.truetype("bedstead.ttf", FONT_SIZE)

raw_sheet = Image.new("RGBA", (IMAGE_SIZE,IMAGE_SIZE), (0,0,0, 255))
draw = ImageDraw.Draw(raw_sheet)
    
ids = {}

for i, character in enumerate(CHARS):
    column = i % COLUMNS
    row = math.floor(i / COLUMNS)
    
    col_pix = column * (IMAGE_SIZE / COLUMNS)
    row_pix = row * (IMAGE_SIZE / ROWS)

    ids[character] = i
    
    color = (255, 0, 0, 255)
    
    draw.text((col_pix,row_pix), character, fill=color, font=FONT)

raw = raw_sheet.load()

# Generate distance field
sdf = Image.new("RGBA", (IMAGE_SIZE,IMAGE_SIZE), (0,0,0, 255))
sdf_raw = sdf.load()

for i in range(IMAGE_SIZE):
    for j in range(IMAGE_SIZE):
        
        d1 = 0 # Outer distance
        d2 = 0 # Inner distance
        
        for x in range(-FILTER_SIZE, FILTER_SIZE):
            for y in range(-FILTER_SIZE, FILTER_SIZE):
                dist = 1 -  ((x**2 + y ** 2) ** 0.5) / (FILTER_SIZE*(2**0.5))
                
                nx = i + x
                ny = j + y
                if nx > 0 and nx < IMAGE_SIZE and ny > 0 and ny < IMAGE_SIZE:
                    pixel = raw[i + x, j + y][0] / 255
                else:
                    pixel = 0.0
                
                d1 = max(d1, pixel * dist)
                d2 = max(d2, (1-pixel) * (dist))

        d = (d1 + (1 - d2)) / 2
        d = int(d * 255)
        
        sdf_raw[i, j] = (0, d, 0, 255)


red = Image.new("RGBA", (IMAGE_SIZE,IMAGE_SIZE), (135,0,0, 255))
ship_sprite = Image.open("ship.png")

SHIP_SCALE = 1.0
SHIP_DISTORT = 5.0 / 9.0 # Characters are 5 wide, 9 high, so are distorted in the shader. This is the anit-distortion

ship_sprite = ship_sprite.transform(
    ship_sprite.size,
    Image.AFFINE, 
    (
        SHIP_SCALE * SHIP_DISTORT, 
        0, 
        ship_sprite.size[0] * 0.25,# * SHIP_SCALE / SHIP_DISTORT / 4, 
        0, 
        SHIP_SCALE, 
        ship_sprite.size[1] * 0.0,# * SHIP_SCALE / 4)
    )
)
ship_sprite = ship_sprite.resize((IMAGE_SIZE, IMAGE_SIZE))


ship_sprite = ImageChops.multiply(ship_sprite, red)

out_image = ImageChops.add(ship_sprite, sdf)

out_image.save("font.png")
out_image.show()

print(i)