#include <iostream>
#include <ostream>
#include <string>
#include <vector>

using namespace std;

// just because you have n calls total doesn't mean it takes O(n) space.
// Consider the below function, which adds adjacent elements between O and n:
int pairSum(int a, int b) { return a + b; }

int pairSumSequence(int n) {
  int sum = 0;
  for (int i = 0; i < n; i++) {
    sum += pairSum(i, i + 1);
  }
  return sum;
}

// Suppose you have an algorithm that has two steps. When do you
// multiply the runtimes and when do you add them?
// void addOrMutliRuntime(array<string, 2> &arrA, array<string, 3> &arrB) {
void addOrMutliRuntime(vector<string> &arrA, vector<string> &arrB) {
  // we do A chunks of work then B chunks of work. Therefore, the total amount
  // of work isO(A + B)
  for (const string &str : arrA) {
    cout << str << "\n";
  }
  for (const string &str : arrB) {
    cout << str << "\n";
  }

  // we do B chunks of work for each element in A. Therefore, the total amount
  // of work is O(A * B)
  for (const string &str1 : arrA) {
    for (const string &str2 : arrB) {
      cout << str1 << "," << str2 << "\n";
    }
  }
}

// Recursive Runtimes
// The space complexity of this algorithm will be O(N). Although we have 0(2^N)
// nodes in the tree total, only O(N) exist at any given time. Therefore, we
// would only need to have O(N) memory available.
int recursiveRuntime(int n) {
  if (n <= 1) {
    return 1;
  }
  for (int i = 0; i <= n; i++) {
    // call another function like:  f(n - 1) + f(n - 1);
    // a lot of people will see the 2 calls for f and jump to O(N^2), this is completely wrong
  }

  return 0;
}

// What is the runtime of the below code?
// This will take O(N) time. The fact that we iterate through the array twice doesn't matter.
void sum(vector<int> arr) {
  int sum = 0;
  int product = 0;
  for (int i = 0; i < arr.size(); i++) {
    sum += arr[i];
  }
  for (int i = 0; i < arr.size(); i++) {
    product *= arr[i];
  }

  cout << "the sum is: " << sum << ", product is: " << product << endl;
}

// What is the runtime of the below code?
// (N-1) + (N-2) + (N-3) + ... + 2 + 1  = 1 + 2 + 3 + ... + N-1
// = sum of 1 throug h N-1  The sum of 1 through N-1 is:   N(N-1)/2.
// so the runtime will be O(N^2)
void printUnorderedPairs(vector<string> arr) {
  for (int i = 0; i < arr.size(); i++) {
    for (int j = i + 1; j < arr.size(); j++) {
      cout << "i is: " << arr[i] << " j is: " << arr[j] << endl;
    }
  }
}

// 100,000 units of work is still constant, so the runtime is 0( ab)
void printUnorderedPairs2(vector<string> arr1, vector<string> arr2) {
  for (int i = 0; i < arr1.size(); i++) {
    for (int j = 0; j < arr2.size(); j++) {
      for (int k = 0; k < 100000; k++) {
        cout << k << "\n";
      }
    }
  }
  cout << "" << endl;
}

// What is the time complexity of this function?
// The for loop will start when x = 2 and end when x*x = n. Or, in other words, it stops
// when x = vn (when x equals the square root of n). This runs in O(square n) time.
bool isPrime(int n) {
  if (n <= 1) {
    return false;
  }

  for (int x = 2; x * x <= n; x++) {
    if (n % x == 0)
      return false;
  }
  return true;
}

int main(int argc, char *argv[]) {
  // There will be roughly O(N) calls to pairSum. However, those calls do not
  // exist simultaneously on the call stack, so you only need O(1) space
  int sum1 = pairSumSequence(10);
  cout << sum1 << "\n";

  vector<string> arrA = {"a", "b", "c"};
  vector<string> arrB = {"1", "2", "3"};
  addOrMutliRuntime(arrA, arrB);

  cout << "" << flush;

  vector<int> int1 = {1, 2, 3, 4, 5, 6};
  vector<int> int2 = {7, 8, 9, 10, 11, 12};

  sum(int1);
  printUnorderedPairs(arrA);

  return 0;
}
