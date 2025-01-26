#!/usr/bin/env python3

import argparse

from font_hanken_grotesk import HankenGroteskBold, HankenGroteskMedium
from font_intuitive import Intuitive
from PIL import Image, ImageDraw, ImageFont

from inky.auto import auto

print("""Inky pHAT/wHAT: Hello... my name is:

Use Inky pHAT/wHAT as a personalised name badge!

""")

def getsize(font, text):
    _, _, right, bottom = font.getbbox(text)
    return (right, bottom)

try:
    inky_display = auto(ask_user=True, verbose=True)
except TypeError:
    raise TypeError("You need to update the Inky library to >= v1.1.0")

parser = argparse.ArgumentParser()
parser.add_argument("--temp", "-t", type=float, required=True, help="The temperature")
parser.add_argument("--highof", "-ho", type=float, required=True, help="High of the day")
parser.add_argument("--rain", "-r", type=int, required=True, help="Percentage of rain")
parser.add_argument("--jacket", "-j", type=str, required=True, help="Jacket recommendation")
args, _ = parser.parse_known_args()

# inky_display.set_rotation(180)
try:
    inky_display.set_border(inky_display.RED)
except NotImplementedError:
    pass

# Figure out scaling for display size

scale_size = 1.0
padding = 0

if inky_display.resolution == (400, 300):
    scale_size = 2.20
    padding = 15

if inky_display.resolution == (600, 448):
    scale_size = 2.20
    padding = 30

if inky_display.resolution == (250, 122):
    scale_size = 1.30
    padding = -5

# Create a new canvas to draw on

img = Image.new("P", inky_display.resolution)
draw = ImageDraw.Draw(img)

# Load the fonts

intuitive_font = ImageFont.truetype(Intuitive, int(22 * scale_size))
hanken_bold_font = ImageFont.truetype(HankenGroteskBold, int(22 * scale_size))
hanken_medium_font = ImageFont.truetype(HankenGroteskMedium, int(20 * scale_size))

# Grab the name to be displayed

temp = args.temp
high_of = args.highof
rain = args.rain
jacket = args.jacket

# Top and bottom y-coordinates for the white strip

y_top = int(inky_display.height * (5.0 / 10.0))
y_bottom = y_top + int(inky_display.height * (4.0 / 10.0))

# Calculate the positioning and draw the temp text

temp_w, temp_h = getsize(hanken_medium_font, "Temp Today: " + str(temp))
temp_x = int(0)
temp_y = 4 + padding
draw.text((temp_x, temp_y), "Temp Today: " + str(temp), inky_display.BLACK, font=hanken_medium_font)

# Calculate the positioning and draw the high of text

highof_w, highof_h = getsize(hanken_medium_font, "High of: " + str(high_of))
highof_x = int(0)
highof_y = (2 + temp_h + padding)
draw.text((highof_x, highof_y), "High of: " + str(high_of), inky_display.BLACK, font=hanken_medium_font)

# Calculate the positioning and draw the rain percentage text

rain_w, rain_h = getsize(hanken_medium_font, "Rain %: " + str(rain))
rain_x = int(0)
rain_y = int(2 + temp_h + highof_h + padding)
draw.text((rain_x, rain_y), "Rain %: " + str(rain), inky_display.BLACK, font=hanken_medium_font)

# Calculate the positioning and draw the jacket recommendation text

jacket_w, jacket_h = getsize(hanken_medium_font, "Jacket: " + jacket)
jacket_x = int(0)
jacket_y = int(6 + temp_h + highof_h + rain_h + padding)
draw.text((jacket_x, jacket_y), "Jacket: " + jacket, inky_display.BLACK, font=hanken_medium_font)

# Display the completed name badge

inky_display.set_image(img)
inky_display.show()
