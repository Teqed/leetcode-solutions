const ordinalOfA = 'a'.codePointAt(0)!; // 97
const lengthOfAlphabet = 'z'.codePointAt(0)! - ordinalOfA + 1; // 26

function groupAnagrams(strs: string[]): string[][] {
	const groupedAnagrams: Record<string, string[]> = {};
	for (const string_ of strs) {
		const frequency = Array.from({length: lengthOfAlphabet}, () => 0);
		for (const char of string_) {
			frequency[char.codePointAt(0)! - ordinalOfA]++;
		}

		const key = frequency.join(',');
		groupedAnagrams[key] = [...(groupedAnagrams[key] ?? []), string_];
	}

	return Object.values(groupedAnagrams);
}
