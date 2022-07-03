morse = {
	"a": ".-",
	"b": "-...",
	"c": "-.-.",
	"d": "-..",
	"e": ".",
	"f": "..-.",
	"g": "--.",
	"h": "....",
	"i": "..",
	"j": ".---",
	"k": "-.-",
	"l": ".-..",
	"m": "--",
	"n": "-.",
	"o": "---",
	"p": ".--.",
	"q": "--.-",
	"r": ".-.",
	"s": "...",
	"t": "-",
	"u": "..-",
	"v": "...-",
	"w": ".--",
	"x": "-..-",
	"y": "-.--",
	"z": "--..",
	
	"1": ".----",
	"2": "..---",
	"3": "...--",
	"4": "....-",
	"5": ".....",
	"6": "-....",
	"7": "--...",
	"8": "---..",
	"9": "-----.",
	"0": "-----",

	" ": "   "
}

def stm(text: str) -> str:
	result = ""
	for char in text.lower():
		result += f"{morse[char]}  "

	return result


def mts(morse_input: str) -> str:
	result = ""
	for char in morse_input.split():
		result += f"{list(morse.keys())[list(morse.values()).index(char)]}"

	return result

print(stm("allis"))
print(mts(".-  .-..  .-..  ..  ..."))
