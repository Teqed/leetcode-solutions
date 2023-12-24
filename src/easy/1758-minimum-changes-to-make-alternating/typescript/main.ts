function minOperations(s: string): number {
	let count = 0;
	for (const [index, character] of s.split('').entries()) {
		if (character === (index % 2).toString()) {
			count++;
		}
	}

	return Math.min(count, (s.length - count));
}
