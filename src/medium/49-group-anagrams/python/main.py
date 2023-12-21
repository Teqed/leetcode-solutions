from collections import defaultdict
from typing import List

ord_a = ord('a')
alphabetLength = (ord('z') - ord_a + 1)

class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        grouped_anagrams = defaultdict(list)
        for word in strs:
            frequency = [0] * alphabetLength
            for char in word:
                frequency[ord(char) - ord_a] += 1
            grouped_anagrams[tuple(frequency)].append(word)
        return list(grouped_anagrams.values())