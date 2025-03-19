package java_library;

/**
 * AssortedMethods
 */
public class AssortedMethods {

  public static String charArrayToString(char[] array) {
    StringBuffer buffer = new StringBuffer(array.length);
    for (char c : array) {
      if (c == 0)
        break;
      buffer.append(c);
    }

    return buffer.toString();
  }
}
