from flask import Flask, send_file
from PIL import Image, ImageDraw, ImageFont


app = Flask(__name__)

curr_ID = 0
FONT = "fonts/josefin.ttf"


def create_new_image(text: str):
	global curr_ID, FONT

	img = Image.open('images/transparent.png')
	drawer = ImageDraw.Draw(img)
	drawer.text((0, 0), text, fill=(255, 0, 0), font=ImageFont.truetype(FONT, 200))
	new_filename = f"images/{curr_ID}.png"
	img.save(new_filename)
	curr_ID += 1

	return new_filename

@app.route("/<text>")
def main(text):
	return send_file(create_new_image(text))

app.run()