import os

coefficientCounter = 1
userCoefficients = []

while True:
	prompt = "Leading Coefficient: " if coefficientCounter == 1 else f"Coefficient {coefficientCounter} or 'n' to stop: "
	coefficientCounter += 1
	try:
		userCoefficients.append(int(input(prompt)))
	except ValueError:
		break

print(userCoefficients, userCoefficients[::-1])
os.system("cls || clear")
equationString = "f(x) ="
counter = len(userCoefficients) - 1
for i in userCoefficients:
	equationString += f" {'+' if counter != len(userCoefficients)-1 else ''} {i}x^{counter}"
	counter -= 1

print(f"Your equation is: {equationString}")

inputs = []

while True:
	try:
		inputs.append(int(input("Enter an input or 'n' to stop: ")))
	except ValueError:
		break
os.system("cls || clear")

for input in inputs:
	counter = len(userCoefficients) - 1
	out = 0
	for coefficient in userCoefficients:
		out = out + coefficient * (input**counter)
		counter -= 1
	print(f"f({input}) =\t {out}")