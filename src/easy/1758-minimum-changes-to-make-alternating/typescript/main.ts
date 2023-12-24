function minOperations(s: string): number {
	let a = 0, b = 0; // eslint-disable-line one-var, one-var-declaration-per-line
	for (const [i, c] of s.split('').entries()) {
		if (c === (i % 2).toString()) {
			a++;
		} else {
			b++;
		}
	}

	return Math.min(a, b);
}
