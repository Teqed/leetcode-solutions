/* eslint-disable curly */
/* eslint-disable unicorn/no-for-loop */
function twoSum(nums: number[], target: number): number[] {
	if (nums.length === 2) return [0, 1];

	const numberIndices = new Map<number, number>();
	for (let i = 0; i < nums.length; i++) {
		const currentNumber = nums[i];
		const complement = target - currentNumber;
		if (numberIndices.has(complement)) return [numberIndices.get(complement)!, i];

		numberIndices.set(currentNumber, i);
	}

	return [];
}
