
// Given an array arr[] of n integers, construct a product array res[] (of the same size)
// such that res[i] is equal to the product of all the elements of arr[] except arr[i].

// Input: arr[] = [10, 3, 5, 6, 2]
// Output: [180, 600, 360, 300, 900]
// Explanation:
//     For i=0, res[i] = 3 * 5 * 6 * 2 is 180.
//     For i = 1, res[i] = 10 * 5 * 6 * 2 is 600.
//     For i = 2, res[i] = 10 * 3 * 6 * 2 is 360.
//     For i = 3, res[i] = 10 * 3 * 5 * 2 is 300.
//     For i = 4, res[i] = 10 * 3 * 5 * 6 is 900.

#include <iostream>
#include <vector>
using namespace std;

// [Naive Approach] Using Nested Loops – O(n^2) Time and O(1) Space
vector<int> productExceptSelf1(vector<int> &arr) {
  int n = arr.size();

  // Fill result array with 1
  vector<int> res(n, 1);
  for (int i = 0; i < n; i++) {
    // Compute product of all elements except arr[i]
    for (int j = 0; j < n; j++) {
      if (i != j)
        res[i] *= arr[i];
    }
  }

  return res;
}

// [Better approach] Using Prefix and Suffix Array – O(n) Time and O(n) Space
vector<int> productExceptSelf(vector<int> &arr) {
  // arr = [1, 2, 3, 4]
  int n = arr.size();

  vector<int> prefProduct(n), suffProduct(n), res(n);

  // Construct the prefProduct array
  // Initialize prefProduct[0] = 1 because there are no elements to the
  // left of the first element.
  prefProduct[0] = 1;
  for (int i = 1; i < n; i++) {
    // For i = 1: prefProduct[1] = arr[0] * prefProduct[0] = 1 * 1 = 1.
    // For i = 2: prefProduct[2] = arr[1] * prefProduct[1] = 2 * 1 = 2.
    // For i = 3: prefProduct[3] = arr[2] * prefProduct[2] = 3 * 2 = 6.
    prefProduct[i] = arr[i - 1] * prefProduct[i - 1];
  }
  // prefProduct = [1, 1, 2, 6]

  // Construct the suffProduct array
  suffProduct[n - 1] = 1;
  for (int j = n - 2; j >= 0; j--) {
    // prefProduct[i] stores product of all elements before i-th index in the array.
    // For j = 2: suffProduct[2] = arr[3] * suffProduct[3] = 4 * 1 = 4.
    // For j = 1: suffProduct[1] = arr[2] * suffProduct[2] = 3 * 4 = 12.
    // For j = 0: suffProduct[0] = arr[1] * suffProduct[1] = 2 * 12 = 24.
    suffProduct[j] = arr[j + 1] * suffProduct[j + 1];
  }
  // suffProduct = [24, 12, 4, 1]

  for (int i = 0; i < n; i++)
    res[i] = prefProduct[i] * suffProduct[i];
  // For i = 0: res[0] = prefProduct[0] * suffProduct[0] = 1 * 24 = 24.
  // For i = 1: res[1] = prefProduct[1] * suffProduct[1] = 1 * 12 = 12.
  // For i = 2: res[2] = prefProduct[2] * suffProduct[2] = 2 * 4 = 8.
  // For i = 3: res[3] = prefProduct[3] * suffProduct[3] = 6 * 1 = 6.

  return res;
}

// [Efficient Approach] Using Product Array – O(n) Time and O(1) Space
vector<int> productExceptSelf3(vector<int> &arr) {
  int zeros = 0, idx = -1;
  int prod = 1;

  // Count zeros and track the index of the zero
  for (int i = 0; i < arr.size(); ++i) {
    if (arr[i] == 0) {
      zeros++;
      idx = i;
    } else {
      prod *= arr[i];
    }
  }

  vector<int> res(arr.size(), 0);

  // If no zeros, calculate the product for all elements if (zeros)
  if (zeros == 0) {
    for (int i = 0; i < arr.size(); i++) {
      res[i] = prod / arr[i];
    }
  } else if (zeros == 1) {
    // // If one zero, set product only at the zero's index
    res[idx] = prod;
  }

  return res;
}

int main() {
  vector<int> arr = {10, 3, 5, 6, 2};
  vector<int> res = productExceptSelf1(arr);
  vector<int> res2 = productExceptSelf1(arr);

  cout << "\n";
  for (int val : res)
    cout << val << " \n";

  for (int val : res2)
    cout << val << " \n";
  return 0;
}
