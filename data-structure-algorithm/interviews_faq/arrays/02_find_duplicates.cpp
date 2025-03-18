//
// Given an array arr[] of n elements that contains elements from 0 to n-1, with any of
// these numbers appearing any number of times. The task is to find the repeating numbers.
//
// Input: n = 7, arr[] = [1, 2, 3, 6, 3, 6, 1]
// Output: 1, 3, 6
// Explanation: The numbers 1 , 3 and 6 appears more than once in the array.
//
// Input : n = 5, arr[] = [1, 2, 3, 4 ,3]
// Output: 3
// Explanation: The number 3 appears more than once in the array.
//

#include <bits/stdc++.h>
#include <unordered_map>
#include <vector>
using namespace std;

// Using hashmap – O(n) Time and O(n) Space
vector<int> findDuplicates(vector<int> &arr) {
  // Step 1: Create an empty unordered map to store element frequencies
  int n = arr.size();
  unordered_map<int, int> freqMap;
  vector<int> result;

  for (int i = 0; i < n; i++) {
    freqMap[arr[i]++];
  }

  // Step 3: Iterate through the hashmap to find duplicates
  for (auto &entry : freqMap) {
    if (entry.second > 1)
      result.push_back(entry.first);
  }

  // Step 4: If no duplicates found, add -1 to the result
  if (result.empty())
    result.push_back(-1);

  return result;
}

// Using Auxiliary Array – O(n) Time and O(n) Space
// Since the numbers inside the array range from 0 to n-1 (inclusive), where n is the
// length of the array, we can utilize an auxiliary array of size n to record the
// frequency of each element. By iterating through we can found the duplicates easily.
vector<int> findDuplicate2(vector<int> &arr) {
  int n = arr.size();
  vector<int> freqArr(n);
  vector<int> result;

  // Step 2: Iterate through the array and count element frequencies
  for (int i = 0; i < n; i++) {
    freqArr[arr[i]]++;
  }

  // Step 3: Iterate through all the possible elements to check duplicates
  for (int i = 0; i < n; i++) {
    if (freqArr[i] > 1) {
      result.push_back(i);
    }
  }

  if (result.empty())
    result.push_back(-1);

  return result;
}

int main(int argc, char *argv[]) {
  vector<int> arr = {1, 6, 5, 2, 3, 3, 2};

  vector<int> duplicates = findDuplicates(arr);

  cout << "Duplicated elements \n";
  for (int element : duplicates) {
    cout << element << " ";
  }

  vector<int> duplicates2 = findDuplicates(arr);

  for (int element : duplicates) {
    cout << element << " ";
  }

  return 0;
}
