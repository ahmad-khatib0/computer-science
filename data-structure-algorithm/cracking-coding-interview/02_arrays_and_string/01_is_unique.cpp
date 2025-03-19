#include <bitset>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

// You should first ask your interviewer if the string is an ASCII string or
// a Unicode string. We'll assume for simplicity the character set is ASCII.
bool is_unique_chars(const string &str) {
  if (str.length() > 128) {
    return false;
  }

  vector<bool> char_set(128);
  for (int i = 0; i < str.length(); i++) {
    int val = str[i];
    if (char_set[val])
      return false;
    char_set[val] = true;
  }

  return true;
}

bool is_unique_chars_extended_ascii(const string &str) {
  // Reduce space usage by a factor of 8 using bitvector.
  // Each boolean otherwise occupies a size of 8 bits.

  // The extended ASCII table has 256 possible characters,
  bitset<256> bits(0);

  for (int i = 0; i < str.length(); i++) {
    int val = str[i];
    if (bits.test(i) > 0)
      return false;
    bits.set(val);
  }

  return true;
}

bool is_unique_chars_no_ds(const string &str) {
  for (int i = 0; i < str.length() - 1; i++) {
    for (int j = i + 1; j < str.length(); j++) {
      if (str[i] == str[j])
        return false;
    }
  }
  return true;
}

int main() {
  vector<string> words = {"abcde", "hello", "apple", "kite", "padle"};
  for (auto word : words) {
    cout << word << string(": ") << boolalpha << is_unique_chars(word) << endl;
  }

  cout << endl << "Using bit vector" << endl;
  for (auto word : words) {
    cout << word << string(": ") << boolalpha << is_unique_chars_extended_ascii(word) << endl;
  }

  cout << endl << "Using no Data Structures" << endl;
  for (auto word : words) {
    cout << word << string(": ") << boolalpha << is_unique_chars_no_ds(word) << endl;
  }

  return 0;
}
