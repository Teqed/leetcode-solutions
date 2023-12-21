// eslint-disable-next-line @typescript-eslint/naming-convention
function topKFrequent(nums: number[], k: number): number[] {
	const freq = new Map<number, number>();

	for (const number_ of nums) {
		freq.set(number_, (freq.get(number_) ?? 0) + 1);
	}

	const buckets = Array.from({length: nums.length + 1}).map(() => []);
	for (const [number_, count] of freq.entries()) {
		(buckets as number[][])[count].push(number_);
	}

	const result: number[] = [];
	for (let i = buckets.length - 1; i >= 0 && result.length < k; i--) {
		result.push(...(buckets as number[][])[i]);
	}

	return result;
}
