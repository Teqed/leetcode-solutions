from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        if (len(nums) == 2): return [0, 1]  # noqa: E701
        number_collection = {}
        for i, num in enumerate(nums):
            if (target - num in number_collection):
                return [number_collection[target - num], i]
            number_collection[num] = i