# Intuition
<!-- Describe your first thoughts on how to solve this problem. -->
There are two possible ways to alternate a string of 0s and 1s. We'll want to calculate how many changes it would take to make the string alternate in each of these ways, and return the minimum of the two.

As we iterate each character, we'll increment a counter for which string would need to alternate at that index.

# Approach
<!-- Describe your approach to solving the problem. -->
We [fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) over the string, using the index and character at each step to determine which string we would need to alternate at that index. For example, if the first character was a 0, we would need to alternate the string starting with a 1 at that index -- otherwise, we would need to alternate the string starting with a 0 at that index.

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
        let (a, b) = s.chars().enumerate().fold((0, 0), |(a, b), (i, c)| {
            if c == (if i % 2 == 0 { '0' } else { '1' }) { (a + 1, b) } else { (a, b + 1) }
        });
        std::cmp::min(a, b)
    }
}
```
## Python
``` Py
class Solution:
    def minOperations(self, s: str) -> int:
        a = 0
        b = 0
        for i in range(len(s)):
            if s[i] != str(i % 2):
                a += 1
            else:
                b += 1
        return min(a, b)
```
## Typescript
``` Typescript
function minOperations(s: string): number {
	let a = 0, b = 0;
	for (const [i, c] of s.split('').entries()) {
		if (c === (i % 2).toString()) {
			a++;
		} else {
			b++;
		}
	}

	return Math.min(a, b);
}
```