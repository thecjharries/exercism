package pangram

import "strings"

func IsPangram(input string) bool {
	var alphabet = "abcdefghijklmnopqrstuvwxyz"
	var alphabetMap = make(map[rune]bool)
	for _, letter := range alphabet {
		alphabetMap[letter] = true
	}
	for _, letter := range strings.ToLower(input) {
		delete(alphabetMap, letter)
	}
	return len(alphabetMap) == 0
}
