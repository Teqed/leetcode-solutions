# Intuition
<!-- Describe your first thoughts on how to solve this problem. -->
There are two possible ways to alternate a string of 0s and 1s. We'll want to calculate how many changes it would take to make the string alternate in each of these ways, and return the minimum of the two.

As we iterate each character, we'll increment a counter if the character would need to be flipped to match the string if it were starting with a 0. At the end, we can take the length of the string minus the count to get the number of changes needed to make the string alternate if it were starting with a 1.

# Approach
<!-- Describe your approach to solving the problem. -->
We [fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) over the string, using the index and character at each step to determine if the character would need to be flipped to match the string if it were starting with a 0.

The index modulo 2 will be 0 if the character should be a 0, and 1 if the character should be a 1. We compare this to the character, and increment our count if they are not equal.

# Complexity
- Time complexity:
<!-- Add your time complexity here, e.g. $$O(n)$$ -->
$$O(n)$$

- Space complexity:
<!-- Add your space complexity here, e.g. $$O(n)$$ -->
$$O(1)$$

# Code
## Rust
``` Rust
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let count = s.chars().enumerate().fold(0, |a, (index, character)| {
            if character == (if index % 2 == 0 { '0' } else { '1' }) { a + 1 } else { a }
        });
        std::cmp::min(count, s.len() as i32 - count)
    }
}
```
## Python
``` Py
class Solution:
    def minOperations(self, s: str) -> int:
        count = 0
        for index in range(len(s)):
            if s[index] != str(index % 2):
                count += 1
        return min(count, (len(s) - count))
```
## Typescript
``` Typescript
function minOperations(s: string): number {
	let count = 0;
	for (const [index, character] of s.split('').entries()) {
		if (character === (index % 2).toString()) {
			count++;
		}
	}

	return Math.min(count, (s.length - count));
}

```