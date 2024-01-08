from typing import List


class Solution:

    escape_character = '\0'

    def encode(self, strs: List[str]) -> str:
        """Encodes a list of strings to a single string, using an escape character to separate the strings."""
        output = ""
        for s in strs:
            output += s + self.escape_character
        return output

    def decode(self, s: str) -> List[str]:
        """Decodes a single string to a list of strings, using an escape character to separate the strings."""
        output = []
        current = ""
        for c in s:
            if c == self.escape_character:
                output.append(current)
                current = ""
            else:
                current += c
        return output

class Tests:
    strs_array = [
        ["basic","string","of","characters"],
        ["dog","cat",":","true"],
        [""],
        [],
        ["fish","fish",":","false","!@#$%^&*()"],
        ["#","##"],
        ["1,23","45,6","7,8,9"],
        ["hello","world","hello"],
        ["a","b","c","d"],
        ["@#$","%^&*","!@#$%^&*"],
        ["apple","orange","banana","grape","kiwi","melon"],
        ["The quick brown fox","jumps over the","lazy dog","1234567890","abcdefghijklmnopqrstuvwxyz"],
        ["","   ","!@#$%^&*()_+","LongStringWithNoSpaces","Another, String With, Commas"],
        ["String with new\nline","Another\nLine","And\nOne\nMore"],
        ["123","456","789","10","11","12","13","14","15","16","17","18","19","20"],
        ["Repeated","Repeated","Repeated","Repeated","Repeated","Repeated"],
        ["SingleCharacter","A","B","C","D","E","F","G","H","I","J"],
        ["EmojiTest 😊","🌟✨🌟✨🌟","🤖👽🤖👽"],
        ["Strings with spaces are tricky","Another one with spaces","This also has spaces"],
        ["VeryLongStringWithoutAnySpacesOrSpecialCharactersRepeatedManyTimesVeryLongStringWithoutAnySpacesOrSpecialCharactersRepeatedManyTimes"]
    ]
    for strs in strs_array:
        encoded = Solution().encode(strs)
        decoded = Solution().decode(encoded)
        print(f"Test: {strs} -> {encoded} -> {decoded}")
        assert decoded == strs, f"Failed test: {strs} != {decoded}"
    print("All tests passed successfully.")

# Run tests
Tests()