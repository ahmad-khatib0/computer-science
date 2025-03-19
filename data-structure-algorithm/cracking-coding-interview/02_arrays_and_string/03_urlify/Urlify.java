package _02_arrays_and_string._03_urlify;

import java_library.AssortedMethods;

//
// URLify: Write a method to replace all spaces in a string with '%20'. You may assume 
// that the string has sufficient space at the end to hold the additional characters, 
// and that you are given the "true" length of the string. (Note: if implementing in 
// Java, please use a character array so that you can perform this operation in place.)
// EXAMPLE
// Input: "Mr John Smith ", 13
// Output: "Mr%20John%20Smith"
//

public class Urlify {

  public static void replaceText(char[] str, int trueLength) {
    int spaceCount = 0, index, i = 0;
    for (i = 0; i < trueLength; i++) {
      if (str[i] == ' ')
        spaceCount++;
    }

    // Each space is replaced with %20, which adds 2 extra characters per space
    // spaceCount * 2 calculates the total extra characters needed.
    index = trueLength + spaceCount * 2;
    // the end of the string is properly terminated with a null character (\0).
    // This is a safety measure to handle cases where the array
    // has more space than needed. (see info.md)
    if (trueLength < str.length)
      str[trueLength] = '\0';
    for (i = trueLength - 1; i >= 0; i--) {
      if (str[i] == ' ') {
        str[index - 1] = '0';
        str[index - 2] = '2';
        str[index - 3] = '%';
        index = index - 3;
        // 6th: ['M', 'r', ' ', 'J', 'o', 'h', 'n', ' ', '%', '2', '0', 'S', 'm', 'i',
        // 't', 'h']
        // 11th: ['M', 'r', '%', '2', '0', 'J', 'o', 'h', 'n', '%', '2', '0', 'S', 'm',
        // 'i', 't', 'h']
      } else {
        // 1st itr:
        // ['M', 'r', ' ', 'J', 'o', 'h', 'n', ' ', 'S', 'm', 'i', 't', 'h', ' ', ' ', '
        // ', 'h']
        // 2nd: ['M', 'r', ' ', 'J', 'o', 'h', 'n', ' ', 'S', 'm', 'i', 't', 'h', ' ', '
        // ', 't', 'h']
        // 3rd: ['M', 'r', ' ', 'J', 'o', 'h', 'n', ' ', 'S', 'm', 'i', 't', 'h', ' ',
        // 'i', 't', 'h']
        // 4th: ['M', 'r', ' ', 'J', 'o', 'h', 'n', ' ', 'S', 'm', 'i', 't', 'h', 'm',
        // 'i', 't', 'h']
        // 5th: ['M', 'r', ' ', 'J', 'o', 'h', 'n', ' ', 'S', 'm', 'i', 't', 'S', 'm',
        // 'i', 't', 'h']
        // 7th: ['M', 'r', ' ', 'J', 'o', 'h', 'n', 'n', '%', '2', '0', 'S', 'm', 'i',
        // 't', 'h']
        // 8th: ['M', 'r', ' ', 'J', 'o', 'h', 'h', 'n', '%', '2', '0', 'S', 'm', 'i',
        // 't', 'h']
        // 9th: ['M', 'r', ' ', 'J', 'o', 'o', 'h', 'n', '%', '2', '0', 'S', 'm', 'i',
        // 't', 'h']
        // 10th: ['M', 'r', ' ', 'J', 'J', 'o', 'h', 'n', '%', '2', '0', 'S', 'm', 'i',
        // 't', 'h']
        str[index - 1] = str[i];
        index--;
      }
    }
  }

  public static int findLastCharacter(char[] str) {
    for (int i = str.length - 1; i >= 0; i--) {
      if (str[i] != ' ') {
        return i;
      }
    }
    return -1;
  }

  public static void main(String[] args) {
    String str = "Mr John Smith    ";
    char[] arr = str.toCharArray();
    int trueLength = findLastCharacter(arr) + 1;
    // The true string is "Mr John Smith" (length 13).
    replaceText(arr, trueLength);
    System.out.println("\"" + AssortedMethods.charArrayToString(arr) + "\"");
  }
}
