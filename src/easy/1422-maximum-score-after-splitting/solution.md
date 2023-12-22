# Intuition
<!-- Describe your first thoughts on how to solve this problem. -->
During our first iteration, we can create a count of ones in the string. Then we can iterate again, using the count of zeros already seen and ones yet to be seen to calculate the score at each index. This would allow us to find the maximum score in about n * 2 time.

# Approach
<!-- Describe your approach to solving the problem. -->
We initialize our result and count of zeroes to 0, and our count of ones to the total number of ones in the string by using [filter](https://doc.rust-lang.org/std/iter/struct.Filter.html) and [count](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count).

Then we iterate through the string while using a [match](https://doc.rust-lang.org/reference/expressions/match-expr.html) expression, which is similar to a switch statement in other languages. We match on each character, and increment our count of zeroes if we see a 0, decrement our count of ones if we see a 1, and do nothing as the default case. See an example of this syntax [here](https://doc.rust-lang.org/rust-by-example/flow_control/match.html).

At each step, our count of zeros (which are left of index) + ones (still remaining right of index) = the score.

We only enumerate to the second to last index, because of the constraint that the left and right parts of the string must be non-empty.

# Complexity
- Time complexity:
<!-- Add your time complexity here, e.g. $$O(n)$$ -->
$$O(n)$$

- Space complexity:
<!-- Add your space complexity here, e.g. $$O(n)$$ -->
$$O(1)$$

# Code
```
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let (mut result, mut zeros, mut ones) = (
            0, 0, s.chars().filter(|&character| character == '1').count() as i32);
        for (_i, character) in s.chars().enumerate().take(s.len() - 1) {
            match character {
                '0' => zeros += 1,
                '1' => ones -= 1,
                _ => {}
            }
            result = result.max(zeros + ones);
        }
        result
    }
}
```
