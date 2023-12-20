function isAnagram(s: string, t: string): boolean {
	if (s.length !== t.length) {
		return false;
	}

	const charCount: Record<string, number> = {};
	for (let i = 0; i < s.length; i++) { // eslint-disable-line unicorn/no-for-loop
		charCount[s[i]] = (charCount[s[i]] || 0) + 1;
		charCount[t[i]] = (charCount[t[i]] || 0) - 1;
	}

	return !Object.values(charCount).some(count => count !== 0);
}
