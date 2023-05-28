#!python
# cubase_launcher.py

import mido
import subprocess
import time
import os
from shutil import copyfile,copytree

def main():
	print("Disable Motu Monitoring...")

	print(mido.get_output_names())
	outport = mido.open_output('Midihub MH-28FJS9W Port 4')
	
	monitor_on = mido.Message('note_on', note=102, velocity=127, channel=4)
	monitor_off = mido.Message('note_on', note=101, velocity=127, channel=4)
	outport.send(monitor_off)
	# outport.send(monitor_on)
	print("Quitting...")

if __name__ == "__main__":
    main()
