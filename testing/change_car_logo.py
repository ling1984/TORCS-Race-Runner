from PIL import Image
import argparse
import os

parser = argparse.ArgumentParser(
    prog='CarLogoEditor',
    description='Input a scr car index and the path to an image to overlay onto the car .rgb file.',)
parser.add_argument('--car_index', type=int, help='The index of the car (0-9)')
parser.add_argument('--image_path', type=str, help='The absolute path to the image to overlay (e.g., patch.png)')

args = parser.parse_args()
car_index = args.car_index
image_path = args.image_path

print("args read: ", args)

# 1st we need the pathing to the car images
# current_dir -> \torcs\drivers\scr_server\0..9\car1-ow1.rgb
copy_path = os.path.join(os.getcwd(), "torcs", "drivers", "scr_server", str(car_index), "car1-ow1 - Copy.rgb")
print("copy path: ", copy_path)

base = Image.open(copy_path)      # .rgb image
overlay = Image.open(image_path)  # Can be any format
print(image_path, " opened successfully.")
overlay = overlay.convert(base.mode) # convert to .rgb

# we have 2 target sizes for 4 regions
# 132x44 (3:1) and 90x30 (3:1)

# scale the image to the two sizes
overlay_132 = overlay.resize((132, 44), Image.NEAREST)
overlay_90 = overlay.resize((90, 30), Image.NEAREST)

# regions
# (159,447), (223,379) and (333, 172), (292, 260)
regions_132 = [(159, 447), (223, 379)]
regions_90 = [(333, 172), (292, 260)]

for region in regions_132:
    base.paste(overlay_132, region, overlay_132) # the 3rd argument is the mask, which allows for transparency

for region in regions_90:
    base.paste(overlay_90, region, overlay_90)

result_path = os.path.join(os.getcwd(), "torcs", "drivers", "scr_server", str(car_index), "car1-ow1.rgb")
base.save(result_path)