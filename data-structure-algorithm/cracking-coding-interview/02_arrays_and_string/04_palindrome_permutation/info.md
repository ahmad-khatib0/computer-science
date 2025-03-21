Input:  Tact Coa
Output: True (permutations: "taco cat'; "atc o eta·; etc.) Why is it true?

The input `"Tact Coa"` is considered valid because it can be rearranged into a palindrome,
such as `"taco cat"` or `"atco cta"`.

Let’s break this down step by step:

---

### What is a Palindrome?
A **palindrome** is a word, phrase, or sequence that reads the same backward as forward. Examples:
- `"madam"`
- `"racecar"`
- `"taco cat"`

---

### What is a Permutation of a Palindrome?
A **permutation of a palindrome** is a rearrangement of the characters of a string that 
  forms a palindrome. For example:
- The string `"Tact Coa"` can be rearranged into `"taco cat"`, which is a palindrome.
- Another permutation could be `"atco cta"`.

---

### Key Insight:
For a string to be a permutation of a palindrome:
1. **All characters must occur an even number of times**, except for **at most one character**
   (which can occur an odd number of times). This odd character would be the middle character
   in the palindrome.

---

### Algorithm to Check for Permutation of a Palindrome:
1. **Ignore spaces and case**:
   - Spaces don’t affect whether a string is a palindrome.
   - Convert all characters to lowercase (or uppercase) to handle case insensitivity.

2. **Count character frequencies**:
   - Use a frequency table (e.g., an array or a hash map) to count how many times each
     character appears in the string.

3. **Check the frequency counts**:
   - If all characters have even counts, the string can form a palindrome.
   - If exactly one character has an odd count, the string can still form a palindrome 
     (this character will be the middle character).
   - If more than one character has an odd count, the string cannot form a palindrome.

---

### Example: `"Tact Coa"`

#### Step 1: Preprocess the String
- Ignore spaces and convert to lowercase:
  ```
  "tactcoa"
  ```

#### Step 2: Count Character Frequencies
- Use a frequency table to count occurrences of each character:
  ```
  t: 2
  a: 2
  c: 2
  o: 1
  ```

#### Step 3: Check Frequency Counts
- Characters with even counts: `t`, `a`, `c`.
- Characters with odd counts: `o` (only one character has an odd count).

Since there is **at most one character with an odd count**, the string `"Tact Coa"` 
is a permutation of a palindrome.

---

### Code Implementation:

Here’s how you can implement this logic in Java:

```java
public class PalindromePermutation {

    public static boolean isPermutationOfPalindrome(String phrase) {
        // Create a frequency table for characters
        int[] table = buildCharFrequencyTable(phrase);
        // Check if the frequency table satisfies the palindrome condition
        return checkMaxOneOdd(table);
    }

    // Build a frequency table for characters (ignoring spaces and case)
    private static int[] buildCharFrequencyTable(String phrase) {
        int[] table = new int[26]; // Assuming only lowercase letters
        for (char c : phrase.toCharArray()) {
            int x = getCharNumber(c);
            if (x != -1) {
                table[x]++;
            }
        }
        return table;
    }

    // Map each character to a number (a -> 0, b -> 1, ..., z -> 25)
    private static int getCharNumber(Character c) {
        int a = Character.getNumericValue('a');
        int z = Character.getNumericValue('z');
        int val = Character.getNumericValue(c);
        if (a <= val && val <= z) {
            return val - a;
        }
        return -1; // Non-letter characters
    }

    // Check if at most one character has an odd count
    private static boolean checkMaxOneOdd(int[] table) {
        boolean foundOdd = false;
        for (int count : table) {
            if (count % 2 == 1) {
                if (foundOdd) {
                    return false; // More than one character with odd count
                }
                foundOdd = true;
            }
        }
        return true;
    }

    public static void main(String[] args) {
        String input = "Tact Coa";
        System.out.println(isPermutationOfPalindrome(input)); // Output: true
    }
}
```

---

### Explanation of the Code:

1. **`buildCharFrequencyTable`**:
   - Creates a frequency table for characters in the string, ignoring spaces and case.
   - Uses the `getCharNumber` method to map characters to indices (`0` for `'a'`, `1` for `'b'`, etc.).

2. **`checkMaxOneOdd`**:
   - Checks if at most one character has an odd count in the frequency table.
   - If more than one character has an odd count, the string cannot be a permutation
     of a palindrome.

3. **`getCharNumber`**:
   - Maps characters to numbers (`'a'` to `0`, `'b'` to `1`, etc.).
   - Returns `-1` for non-letter characters (e.g., spaces).

---

### Output:
For the input `"Tact Coa"`, the output is `true` because it can be rearranged into 
a palindrome like `"taco cat"` or `"atco cta"`.

---

### Key Takeaways:
- A string is a permutation of a palindrome if:
  - All characters have even counts, or
  - Exactly one character has an odd count.
- The algorithm works by counting character frequencies and checking the above condition.

