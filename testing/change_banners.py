from PIL import Image
import argparse
import os
import sys
import shutil

parser = argparse.ArgumentParser(
    prog='BannerEditor',
    description='Change the banners on the Corkscrew track.',)
parser.add_argument('--banner_img', type=str, help='The absolute path to the new banner image.')

args = parser.parse_args()
banner_img_path = args.banner_img

corkscrew_path = os.path.join(os.path.dirname(os.path.realpath(sys.argv[0])), "torcs", "tracks", "road", "corkscrew")


if banner_img_path:
    truck_file = "treeRNS2.png"
    truck_outline = "treeRNS2_outline.png"
    truck_parts = ["Body", "Back"]
    truck_res = [(176, 54), (36, 47)]
    truck_coords = [[(0, 6), (0, 67), (0, 134), (0, 195)], [(182, 10), (182, 138)]]

    ## handle truck one in loop

    for part, res, coords_list in zip(truck_parts, truck_res, truck_coords):
        base_path=os.path.join(corkscrew_path, truck_file)
        base = Image.open(base_path)

        # prep banner img
        banner_img=Image.open(banner_img_path)
        banner_img.convert(base.mode)
        banner_img_resized=banner_img.resize(res, Image.NEAREST)

        for pos in coords_list:
            base.paste(banner_img_resized, pos, banner_img_resized)

        # now put the outline on
        outline_path=os.path.join(corkscrew_path, truck_outline)
        outline_img=Image.open(outline_path)
        base.paste(outline_img, mask=outline_img)

        base.save(base_path)

    # handle rest of files

    files = ["kilo.png", "TRUCK07.png", "64PASS1.png", "64PASS6.png"]
    target_res = [(512, 256), (128, 64), (512, 123), (512, 158)]
    start_coords = [(0, 0), (0, 64), (0, 228), (0, 0)]


    for file, res, pos in zip(files, target_res, start_coords):
        base_path=os.path.join(corkscrew_path, file)
        base = Image.open(base_path)

        # prep banner img
        banner_img=Image.open(banner_img_path)
        banner_img.convert(base.mode)
        banner_img_resized=banner_img.resize(res, Image.NEAREST)

        base.paste(banner_img_resized, pos, banner_img_resized)
        base.save(base_path)

else:
    files = ["kilo", "TRUCK07", "64PASS1", "64PASS6", "treeRNS2"]
    for file in files:
        copy_path = os.path.join(corkscrew_path, file+"_copy.png")
        original_path = os.path.join(corkscrew_path, file+".png")
        shutil.copy(copy_path, original_path) # replace original with copy (reset)
