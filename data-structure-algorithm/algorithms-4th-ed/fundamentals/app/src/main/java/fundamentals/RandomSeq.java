package fundamentals;

// import ;

import java.util.Random;

public class RandomSeq {
  public static void randomSeq(String[] args) {
    int n = Integer.parseInt(args[0]);
    double lo = Double.parseDouble(args[1]);
    double hi = Double.parseDouble(args[2]);

    for (int i = 0; i < n; i++) {
      double x = StdRandom.uniform(lo, hi);
      System.out.printf("%.2f\n", x);
    }
  }
}
