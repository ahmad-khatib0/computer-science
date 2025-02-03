// Given a string, find the length of the longest substring without repeating characters.
// for example  =>  "abccabb", so ( abc ) is a portion, ( cab )  a second portion, b is third, so the
// longest without repeating characters is ( 3 ),  don't consider the doublecation
// another example is  "cccccc"  , the answer is 1, becuase c is repeativite

// 1- is the string contiguous ?
// yes look for a substring not a subsequence
// 2- does the case sensitivity matter ?
// No , assueme all the characters in the string are lowercase
// at the end will come up with a solution to cover overlaping, e,g "abcbda"  here the 1th portion is the 1,2,3 postions,
// the 2th portion is from 3,4,5,6 , so they overlaping on the 3th postion, btw the answer here is 4
