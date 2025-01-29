package fundamentals;

import java.util.Arrays;

// java BinarySearch tinyW.txt < tinyT.txt
public class BinarySearch {

  private static int _rank(int key, int[] a) {
    int lo = 0;
    int hi = a.length - 1;

    while (lo <= hi) {
      // Key is in a[lo..hi] or not present.
      int mid = lo + (hi - lo) / 2;
      if (key < a[mid])
        hi = mid - 1;
      else if (key > a[mid])
        lo = mid + 1;
      else
        return mid;
    }
    return -1;
  }

  public static void rank(String[] args) {
    int[] whitelist = In.readInts(args[0]);
    Arrays.sort(whitelist);

    while (!StdIn.isEmpty()) {
      int key = StdIn.readInt();
      if (_rank(key, whitelist) < 0)
        StdOut.println(key);
    }
  }
}
