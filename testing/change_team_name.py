import argparse
import os

parser = argparse.ArgumentParser(
    prog='ScrTeamNameEditor',
    description='Input a scr car index and the team name.',)
parser.add_argument('--car_index', type=int, help='The index of the car (0-9)')
parser.add_argument('--team_name', type=str, help='The name of the team.')

args = parser.parse_args()
car_index = args.car_index or 0
team_name = args.team_name or "scr_driver 0"
scr_server_xml_path = os.path.join(os.getcwd(), "torcs", "drivers", "scr_server", "scr_server.xml")

lines=[]
# name attribute starts at line 18 (index 17) and continues every 11 lines for each car
with open(scr_server_xml_path, 'r') as file:
    lines = file.readlines()
    name_line_index = 17 + (car_index * 11)
    
    # format is <attstr name="name" val="scr_server 1"></attstr>
    # so if we split by " we want to change the 4th element (index 3) 
    line_parts = lines[name_line_index].split('"')
    line_parts[3] = team_name
    lines[name_line_index] = '"'.join(line_parts)
    
with open(scr_server_xml_path, 'w') as file:
    file.writelines(lines)