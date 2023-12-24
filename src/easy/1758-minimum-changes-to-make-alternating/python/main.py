class Solution:
    def minOperations(self, s: str) -> int:
        count = 0
        for index in range(len(s)):
            if s[index] != str(index % 2):
                count += 1
        return min(count, (len(s) - count))