from collections import Counter, defaultdict
from typing import List


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        frequency = Counter()
        for num in nums:
            frequency[num] += 1
        
        buckets = defaultdict(list)
        for [number_, count] in frequency.items():
            buckets[count].append(number_)

        result = []
        for count in range(len(nums), 0, -1):
            if count in buckets:
                result += buckets[count]
                if len(result) >= k:
                    break

        return result