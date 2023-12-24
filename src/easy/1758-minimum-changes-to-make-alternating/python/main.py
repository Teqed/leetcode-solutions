class Solution:
    def minOperations(self, s: str) -> int:
        s1, s2 = s[::2], s[1::2]
        return min(s1.count('0') + s2.count('1'), s1.count('1') + s2.count('0'))