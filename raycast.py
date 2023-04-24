#!/usr/bin/env python3

# Required parameters:
# @raycast.schemaVersion 1
# @raycast.title PolySolver
# @raycast.mode compact

# Optional parameters:
# @raycast.icon 🔢
# @raycast.argument1 { "type": "text", "placeholder": "Function Coefficients" }
# @raycast.argument2 { "type": "text", "placeholder": "Inputs"}

# Documentation:
# @raycast.description Solves a polynomial function at a range of given inputs.
# @raycast.author Ben Raz
# @raycast.authorURL github.com/benraz123

import pyperclip
import sys

import os

coefficientCounter = 1
try:
    userCoefficients = [float(coefficient) for coefficient in sys.argv[1].split(",")]
except ValueError:
    print(f"{sys.argv[2]} is an invalidly formatted input list")
try:
    inputs = [float(input) for input in sys.argv[2].split(",")]
except ValueError:
    print(f"{sys.argv[2]} is an invalidly formatted input list.")

myString = ""

for userInput in inputs:
	counter = len(userCoefficients) - 1
	out = 0
	for coefficient in userCoefficients:
		out = out + coefficient * (userInput**counter)
		counter -= 1
	myString += f"f({userInput}) = {out}\n"

pyperclip.copy(myString)
print("Succesfully Copied answer!")
