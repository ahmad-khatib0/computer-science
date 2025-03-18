// Given an array prices[] of length N, representing the prices of the stocks
// on different days, the task is to find the maximum profit possible by buying
// and selling the stocks on different days when at most one transaction is allowed.
// Here one transaction means 1 buy + 1 Sell.
//

// Input: prices[] = {7, 10, 1, 3, 6, 9, 2}
// Output: 8
// Explanation: Buy for price 1 and sell for price 9.

// Input: prices[] = {7, 6, 4, 3, 1}
// Output: 0
// Explanation: Since the array is sorted in decreasing order,
// 0 profit can be made without making any transaction.

// Input: prices[] = {1, 3, 6, 9, 11}
// Output: 10
// Explanation: Since the array is sorted in increasing order, we can make
// maximum profit by buying at price[0] and selling at price[n-1]
//

#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// [Naive Approach] By exploring all possible pairs – O(n^2) Time and O(1) Space
int maxProfitNaive(vector<int> &prices) {
  int n = prices.size();
  int res = 0;

  for (int i = 0; i < n - 1; i++) {   //  the day to buy
    for (int j = i + 1; j < n; j++) { // the day to sell
      res = max(res, prices[j] - prices[i]);
    }
  }
  return res;
}

// [Expected Approach] One Traversal Solution – O(n) Time and O(1) Space
int maxProfit(vector<int> &prices) {
  int minSoFar = prices[0], res = 0;
  for (int i = 1; i < prices.size(); i++) {

    // Update the minimum value seen so far if we see smaller
    minSoFar = min(minSoFar, prices[i]);
    // Update result if we get more profit
    res = max(res, prices[i] - minSoFar);
  }

  return res;
}

int main(int argc, char *argv[]) {
  vector<int> prices = {7, 10, 1, 3, 6, 9, 2};
  cout << maxProfitNaive(prices) << endl;

  cout << maxProfit(prices) << endl;
  return 0;
}
