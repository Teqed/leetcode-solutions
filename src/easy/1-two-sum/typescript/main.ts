/* eslint-disable curly */
/* eslint-disable unicorn/no-for-loop */
function twoSum(nums: number[], target: number): number[] {
	if (nums.length === 2) return [0, 1];

	const numberCollection = new Map<number, number>();
	for (let i = 0; i < nums.length; i++) {
		const currentNumber = nums[i];
		const complement = target - currentNumber;
		if (numberCollection.has(complement)) return [numberCollection.get(complement)!, i];

		numberCollection.set(currentNumber, i);
	}

	return [];
}
