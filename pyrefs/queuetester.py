#Project URL for information/contact https://github.com/krixgris/MidiToOSC
#

import mido
import OSC
import json
import math

from datetime import datetime

import easingalgos as easing

import configHandler
import httpHandler


#region TODO
#
#	Feature to throttle events/actions for reicipients that can't handle them fast enough
#	Example being Motu 828es which can't process OSC fast enough for a smooth experience, so throttling OSC being sent might help
#endregion


#region GLOBALS

#can be set to anything you want
configFile = 'oscconfig.json'
# constant for doing calculations for scaling values
# keep note this is 7-bit midi only, no support for nrpn midi yet
midiMaxValue = 127.0


conf = configHandler.configHandler(configFile=configFile)
osc = OSC.OSCClient()
http = httpHandler.httpHandler()
#endregion




# MidiEventList = dict()
# for midiType in conf.definedMidi:
#     MidiEventList[midiType] = dict()
#     for midiNum in conf.definedMidi[midiType]:
# 		MidiEventList[midiType][midiNum] = MidiEvent(midiNum,midiType)

#generic MidiEvent-getter, can replace getAttribute/Type/Command etc
#returns a configHandler.MidiEvent

#deprecated function. still works, but very inefficient
def MidiEvent(midiNum, midiType):
	#with no parameters, configHandler.MidiEvent() returns a dummy event
	midiEvent = configHandler.MidiEvent()
	if (midiType == 'control_change'):
		midiEvent = configHandler.MidiEvent(conf.control_change.get(midiNum))
	if (midiType == 'note_on'):
		midiEvent = configHandler.MidiEvent(conf.note_on.get(midiNum))
	if (midiType == 'note_off'):
		midiEvent = configHandler.MidiEvent(conf.note_off.get(midiNum))
	return midiEvent


#region Internal Commands that can be used for mtoCommand()
#add support for loading custom config via config-parameters configName='oscConfig', configFile='oscconfig.json' ?
def reloadConfig():
	global conf
	conf = configHandler.configHandler(configFile=configFile)
	reconnectOSC()
	reconnectHTTP()
	print(str(datetime.now()) + " Configuration updated")

	#ugly, make a nicer result at some point?
	print conf

def quitViolently(message = 'Quitting violently!'):
	print message
	quit()

#endregion


#reconnects the OSC object to ip/port in config
def reconnectOSC():
	osc.connect((conf.IP,conf.port))
#reconnects HTTP object. in reality just sets the IP to connect to when message is sent
#rename?
def reconnectHTTP():
	http.setIP(conf.IP)

#simply returns midi channel in a -1 way to 'correct' 0 index values, which may confuse a user for debugging purposes
#inconsistent use throughout, may re-evaluate
def getMIDIInputChannel():
	#todo: defaults?, 0?
	return conf.midiChannelInput-1

#region Handlers for events
#generic handler?
def mtoAction(midiNum, midiValue, midiType):
	midiEventType = conf.MidiEventList[midiType][midiNum].type
	if(midiEventType == 'osc'):
		mtoOSC(midiNum, midiValue, midiType)
	if(midiEventType == 'http'):
		mtoHTTP(midiNum, midiValue, midiType)
	if(midiEventType == 'command'):
		mtoCommand(midiNum, midiValue, midiType)

def mtoOSC(midiNum, midiValue, midiType):
	osc.send(getOSCMessage(midiNum, midiValue, midiType))

#implement sending json of multiple values?
#maybe?
#maybe extension of handling multiple events per incoming midi uses json instead of multiple events for http for performance
def mtoHTTP(midiNum, midiValue, midiType):
	#http can technically send batches of data with json, but only one parameter is currently supported
	data = http.getValueList(getHTTPValueAttribute(midiNum,midiValue,midiType), getEventValue(midiNum,midiValue,midiType))
	#print data
	#print getHTTPAddress(midiNum,midiType)
	http.patchData(getEventAddress(midiNum,midiType), data)

def mtoCommand(midiNum, midiValue, midiType):
	midiEventCommand = conf.MidiEventList[midiType][midiNum].command
	conf.MidiEventList[midiType][midiNum]
	if(midiEventCommand == 'reloadConfig'):
		reloadConfig()
	if(midiEventCommand == 'quitLoop'):
		quitViolently()

#endregion

#region Helper functions for getting attributes for mtoEventhandlers

def getHTTPValueAttribute(midiNum, midiValue, midiType):
	# valAttr = MidiEvent(midiNum, midiType).attribute
	valAttr = conf.MidiEventList[midiType][midiNum].attribute
	if (valAttr is None):
		valAttr = 'value'
	return valAttr

def getValueScale(algorithm, value, base = 1):
	scale = 1.0
	#easeInCirc works great for motu 828 es volume

	if algorithm == 'exp':
		scale = (pow(base,value)-1)/(base-1)
	elif algorithm == 'log':
		scale = (math.log(1 + (base-1)*value)/math.log(base))
	
	elif (algorithm == 'easeInSine'):
		scale = easing.easeInSine(value)
	elif (algorithm == 'easeInCubic'):
		scale = easing.easeInCubic(value)
	elif (algorithm == 'easeInQuint'):
		scale = easing.easeInQuint(value)
	elif (algorithm == 'easeInCirc'):
		scale = easing.easeInCirc(value)
	elif (algorithm == 'easeInQuad'):
		scale = easing.easeInQuad(value)
	elif (algorithm == 'easeInQuart'):
		scale = easing.easeInQuart(value)
	elif (algorithm == 'easeInExpo'):
		scale = easing.easeInExpo(value)

	elif (algorithm == 'easeOutSine'):
		scale = easing.easeOutSine(value)
	elif (algorithm == 'easeOutCubic'):
		scale = easing.easeOutCubic(value)
	elif (algorithm == 'easeOutQuint'):
		scale = easing.easeOutQuint(value)
	elif (algorithm == 'easeOutCirc'):
		scale = easing.easeOutCirc(value)
	elif (algorithm == 'easeOutQuad'):
		scale = easing.easeOutQuad(value)
	elif (algorithm == 'easeOutQuart'):
		scale = easing.easeOutQuart(value)
	elif (algorithm == 'easeOutExpo'):
		scale = easing.easeOutExpo(value)

	elif (algorithm == 'easeInOutSine'):
		scale = easing.easeInOutSine(value)
	elif (algorithm == 'easeInOutCubic'):
		scale = easing.easeInOutCubic(value)
	elif (algorithm == 'easeInOutQuint'):
		scale = easing.easeInOutQuint(value)
	elif (algorithm == 'easeInOutCirc'):
		scale = easing.easeInOutCirc(value)
	elif (algorithm == 'easeInOutQuad'):
		scale = easing.easeInOutQuad(value)
	elif (algorithm == 'easeInOutQuart'):
		scale = easing.easeInOutQuart(value)
	elif (algorithm == 'easeInOutExpo'):
		scale = easing.easeInOutExpo(value)

	elif (algorithm == 'easeInBounce'):
		scale = easing.easeInBounce(value)
	elif (algorithm == 'easeOutBounce'):
		scale = easing.easeOutBounce(value)
	elif (algorithm == 'easeInOutBounce'):
		scale = easing.easeInOutBounce(value)
		
	return scale



def getEventValue(midiNum, midiValue, midiType):
	valMin = float(conf.MidiEventList[midiType][midiNum].min)
	valMax = float(conf.MidiEventList[midiType][midiNum].max)
	valScaling = conf.MidiEventList[midiType][midiNum].valueScaling
	valScalingBase = conf.MidiEventList[midiType][midiNum].valueScalingBase

	if valScaling is None:
		valScaling = 'lin'
		valScalingBase = 1
	#value scaling base for log/exp of 20 seems okay for most things so far
	#definitely try different bases for your particular application
	# if(valScaling == 'exp'):	
	# 	eventValue = (valMax-valMin)*(pow(valScalingBase,(midiValue/midiMaxValue))-1)/(valScalingBase-1)+valMin
	# elif(valScaling == 'log'):
	# 	eventValue = (valMax-valMin)*(math.log(1 + (scaleBase-1)*midiValue/midiMaxValue)/math.log(scaleBase))+valMin
	# else:
	# 	#calculate linearly unless exp or log is defined
	# 	if(midiValue == 0):
	# 		eventValue = valMin
	# 	else:
	# 		eventValue = (valMax-valMin)/midiMaxValue*(midiValue+valMin)

	value = midiValue/midiMaxValue
	eventValue = (valMax-valMin)*getValueScale(valScaling, value, valScalingBase)+valMin
	if (midiValue == 0):
		eventValue = valMin
	return eventValue

def getEventAddress(midiNum, midiType):
	address = conf.MidiEventList[midiType][midiNum].address
	return address

def getOSCMessage(midiNum, midiValue, midiType):
	oscMsg = OSC.OSCMessage()
	oscMsg.setAddress(getEventAddress(midiNum, midiType))
	oscMsg.append(getEventValue(midiNum, midiValue, midiType))
	return oscMsg

#endregion

#region Helper functions for MIDI messages, including checks if event is defined

#unused, probably getting removed
def isDefinedMidiLookup(midiNum, midiType, midiCh = -1):
    if(midiCh == conf.midiChannelInput or midiCh == -1):
        if(midiType in conf.definedMidi.keys()):
            if(midiNum in conf.definedMidi[midiType]):
                return True
    return False

def getMidiValue(msg):
	if(msg.type == 'control_change'):
		return msg.value
	if(msg.type == 'note_on' or msg.type == 'note_off'):
		return msg.velocity
	return -1

def getMidiNum(msg):
	if(msg.type == 'control_change'):
		return msg.control
	if(msg.type == 'note_on' or msg.type == 'note_off'):
		return msg.note
	return -1

def isDefinedMidi(msg):
	if(getattr(msg, 'channel', None)==getMIDIInputChannel()):
		if(msg.type in conf.definedMidi.keys()):
			if(getMidiNum(msg) in conf.definedMidi[msg.type]):
				return True
	return False

def isValidMidiInput(inputDevice):
	if conf.midiDeviceInput in mido.get_input_names():
		return 1
	else:
		return 0

#endregion

#region debug functions
#put debug messages to run on launch here
#will only work if debug:1 is set in the json-config
def debugCommands():
	print ''
	print 'Debug messages from debugCommand():'
	print ''
	# mtoAction(80,0,'control_change')

	mtoAction(80,0,'control_change')
	mtoAction(80,1,'control_change')
	mtoAction(80,2,'control_change')
	mtoAction(80,3,'control_change')
	mtoAction(80,4,'control_change')
	mtoAction(80,5,'control_change')
	mtoAction(80,6,'control_change')
	mtoAction(80,127,'control_change')
#endregion

#region Inits

#reloads config, which also sets connections for http, osc
#entire config object is re-initialized

reloadConfig()

#use the information provided from the output here if you struggle with finding a working midi device
#
#for future development, mido.get_output_names() would be useful to add here
print ""
print "Available MIDI Inputs: "
print mido.get_input_names()
print ""

print "Listening on device: "
print conf.midiDeviceInput
print "Listening on channel (0-15), i.e. 0 = midi 1, 15 = midi 16 etc: "
print conf.midiChannelInput

#debug things here
if(conf.debug == 1):
	debugCommands()

#endregion
import time

waitTime = 0.05

midiNum = 21

divisor = 4
midiTypeEvent = 'control_change'
midiValueList = list(range(0,128))
midiMinMax = [0,127]
midiThrottledList = (list(set(midiValueList[0::divisor]+midiMinMax)))

midiThrottledListReverse = list(midiThrottledList[1:])
midiThrottledListReverse.reverse()
print midiThrottledList
print midiThrottledListReverse
while(True):
	for i in midiThrottledList+midiThrottledListReverse:
		# print i
		midiValue = i
		print midiValue
		#mtoOSC(midiNum, midiValue, midiTypeEvent)
		time.sleep(waitTime)
	# for i in midiThrottledListReverse:
	# 	# print abs(i)
	# 	midiValue = i
	# 	#print midiValue
	# 	#mtoOSC(midiNum, midiValue, midiTypeEvent)
	# 	#mtoOSC(midiNum, midiValue, midiTypeEvent)
	# 	time.sleep(waitTime)


#region main loop

# if(isValidMidiInput(conf.midiDeviceInput)):

# 	with mido.open_input(conf.midiDeviceInput) as inport:
# 		for msg in inport:
# 			if(isDefinedMidi(msg)):
# 				mtoAction(getMidiNum(msg), getMidiValue(msg), msg.type)
# 				if(conf.debug == 1):
# 					print "Handled MIDI:"
# 					print(msg)
# 			else:
# 				#debug handling to control printing of messages
# 				#ALL messages gets printed here
# 				if(conf.debug == 1):
# 					print "Unhandled MIDI:"
# 					print(msg)
# else:
# 	quitViolently("MIDI Device could not be found, make sure config matches one of the available MIDI Inputs.")
#endregion