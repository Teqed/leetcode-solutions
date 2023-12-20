function containsDuplicate(nums: number[]): boolean {
	const set = new Set<number>();
	for (const number_ of nums) {
		if (set.has(number_)) {
			return true;
		}

		set.add(number_);
	}

	return false;
}
