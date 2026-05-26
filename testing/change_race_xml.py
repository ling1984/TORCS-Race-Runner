import argparse
import os
import sys

parser = argparse.ArgumentParser(
    prog='RaceXmlEditor',
    description='Input the number of scr drivers to put in the xml file.',)
parser.add_argument('--num_drivers', type=int, help='The number of scr drivers to put in the xml file.')

args = parser.parse_args()
num_drivers = args.num_drivers or 1
scr_server_xml_path = os.path.join(os.path.dirname(os.path.realpath(sys.argv[0])), "torcs", " config", "raceman", "quickrace.xml")

"""
Decided not to implement this feature, as it could interfere with what users are trying to do
(if they want non-scr ai racers or human racers for example).

Format is as follows if needed:
<section name="Drivers">
    <attnum name="maximum number" val="40"/>
    <attnum name="focused idx" val="0"/>
    <attstr name="focused module" val="scr_server"/>

    <section name="1">
        <attnum name="idx" val="0"/>
        <attstr name="module" val="scr_server"/>
    </section>

    <section name="2">
        <attnum name="idx" val="1"/>
        <attstr name="module" val="scr_server"/>
    </section>

</section>


We would add a section for each scr driver up to num_drivers.
"""




# lines=[]
# # name attribute starts at line 18 (index 17) and continues every 11 lines for each car
# with open(scr_server_xml_path, 'r') as file:
#     lines = file.readlines()
#     name_line_index = 17 + (car_index * 11)
    
#     # format is <attstr name="name" val="scr_server 1"></attstr>
#     # so if we split by " we want to change the 4th element (index 3) 
#     line_parts = lines[name_line_index].split('"')
#     line_parts[3] = team_name
#     lines[name_line_index] = '"'.join(line_parts)
    
# with open(scr_server_xml_path, 'w') as file:
#     file.writelines(lines)