#include "string"
#include <algorithm>
#include <iostream>

using namespace std;

// Check Permutation: Given two strings, write a method to
// decide if one is a permutation of the other.

// We will assume for this problem that the comparison is case
// sensitive and whitespace is significant

// Solution #1: Sort the strings.
// If two strings are permutations, then we know they have the same characters, but in
// different orders. Therefore, sorting the strings will put the characters from two
// permutations in the same order
//
bool is_permutation1(string str1, string str2) {
  int n1 = str1.length();
  int n2 = str2.length();

  // If length of both strings is not same, then they cannot be anagram
  if (n1 != n2)
    return false;

  sort(str1.begin(), str1.end());
  sort(str2.begin(), str2.end());

  for (int i = 0; i < n1; i++) {
    if (str1[i] != str2[i])
      return false;
  }

  return true;
}

// Solution #2 (efficient): Check if the two strings have identical character counts.
bool is_permutation2(const string &str1, const string &str2) {
  if (str1.length() != str2.length())
    return false;

  int count[256] = {0}; // Assumption
  for (int i = 0; i < str1.length(); i++) {
    int val = str1[i];
    count[val]++;
  }

  for (int i = 0; i < str2.length(); i++) {
    int val = str2[i];
    count[val]--;
    if (count[val] < 0) {
      return false;
    }
  }

  return true;
}

int main() {
  // Test Method 1 - Using sort
  cout << "Method 1 - Using sort" << endl;
  string str1 = "testest";
  string str2 = "estxest";

  if (is_permutation1(str1, str2))
    cout << str1 << " and " << str2 << " are permutation of each other" << endl;
  else
    cout << str1 << " and " << str2 << " are not permutation of each other" << endl;

  str1 = "hello";
  str2 = "oellh";
  if (is_permutation1(str1, str2))
    cout << str1 << " and " << str2 << " are permutation of each other" << endl;
  else
    cout << str1 << " and " << str2 << " are not permutation of each other" << endl;

  // Test Method 2 - Using character count
  cout << "Method 2 - Using character count" << endl;
  str1 = "testest";
  str2 = "estxest";
  if (is_permutation2(str1, str2))
    cout << str1 << " and " << str2 << " are permutation of each other" << endl;
  else
    cout << str1 << " and " << str2 << " are not permutation of each other" << endl;

  str1 = "hello";
  str2 = "oellh";
  if (is_permutation2(str1, str2))
    cout << str1 << " and " << str2 << " are permutation of each other" << endl;
  else
    cout << str1 << " and " << str2 << " are not permutation of each other" << endl;

  return 0;
}
