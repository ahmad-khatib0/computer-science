// Big notation is the language we use for talking about how long an algorithm takes to run.
// We can compare it to different algorithms or in this case, functions using big O and say which one
// is better than the other when it comes to scale.  Regardless of our computer differences.

const nemo = ['nemo'];
const largeArray = new Array(30000).fill('nemo');

function findNemo(array) {
  // let start = performance.now();
  for (let i = 0; i < array.length; i++) {
    if (array[i] == 'nemo') console.log('found nemo');
  }
  // let end = performance.now();
  // console.log(` function took about ${end - start} milliseconds`);
}

findNemo(largeArray); // O(n) => linear time   complexity: fair

const boxes = [1, 2, 3, 4, 5, 6];
function logFirstTowBoxes(array) {
  console.log(array[0]);
  console.log(array[1]);
}
logFirstTowBoxes(boxes); // O(2) => constant time ,  complexity: excellent
// note: all O(1) or 2 or 3 or any constant time. considered as O(1)

// What is the Big O of the below function?
function exercise(input) {
  let a = 10; // O(1)
  a = 50 + 3; // O(1)

  for (let i = 0; i < input.length; i++) {
    // O(n)
    anotherFunction(); //O(n)  //its like so because its depends on how much input length is
    let stranger = true; //O(n)
    a++; //O(n)
  }
  return a; //O(1)
}

exercise(); // 3 x O(1)  => 3  , and  4 x O(n) => 4 O(n)  ,SO ITS BIG O:     O(3 + 4n)

// What is the Big O of the below function? (Hint, you may want to go line by line)
function exercise2(input) {
  let a = 5; // O(1)
  let b = 10; // O(1)
  let c = 50; // O(1)
  for (let i = 0; i < input; i++) {
    // O(n)
    let x = i + 1; // O(n)
    let y = i + 2; // O(n)
    let z = i + 3; // O(n)
  }
  for (let j = 0; j < input; j++) {
    // O(n)
    let p = j * 2; // O(n)
    let q = j * 2; // O(n)
  }
  let whoAmI = "I don't know"; // O(1)
}

exercise2(); // 4 x O(1)  => 4  , and  7 x O(n) => 7 O(n)  ,SO ITS BIG O:     O(4 + 7n)

////////////////////////////Simplifying Big O ///////////////////
// O(4 + 7n)    => simplified to be O(n)
// so what is the rules to simplify these BigO calculation?

// Rule 1: Always worst Case
// Rule 2: Remove Constants
// Rule 3: Different inputs should have different variables. O(a+b). A and B arrays nested would be
// O(a*b)
// + for steps in order
// * for nested steps
// Rule 4: Drop Non-dominant terms

// Rule 1: Always worst Case
const people = ['ahmad', 'ali', 'moon', 'khalid', 'john', 'sara', 'joe'];
function findJoe(array) {
  for (let i = 0; i < array.length; i++) {
    if (array[i] == 'joe') {
      console.log('found joe');
      break;
    }
  }
}
findJoe(); //, if joe was O(1) or O(5) or o(7) which is the end of array,  it doesn't matter
// because it can be any O notation position , we simplify it to BE =>   O(n)
// so  ALWAYS WORST CASE

// Rule 2: Remove Constants
function printFirstItemThenFirstHalfThenSayHi100Times(items) {
  console.log(items[0]);
  var middleIndex = Math.floor(items.length / 2);
  var index = 0;

  while (index < middleIndex) {
    console.log(items[index]);
    index++;
  }

  for (var i = 0; i < 100; i++) {
    console.log('hi');
  }
}
// BigO is  => O(1 + n/2  + 100 ) // n/2 because we DIVIDE ITEMS be 2 in the middleIndex
// simplified to =>  O( n/2 + 101 )  then simplified to =>   O( n/2 + 1 )  then
// simplified to =>  O( n + 1 )  simplified to => O(n) because e,g adding 1 to 10000 doesn't matter

function compressBoxesTwice(boxes) {
  boxes.forEach((box) => console.log(box));
  boxes.forEach((box) => console.log(box));
}
// O(2n)  => Drop The Constants  => O(n)
// so ALWAYS DROP THE CONSTANTS , WE ARE NOT GOING TO SEE CONSTANTS IN o()

// 3 - Different terms for inputs
function compressBoxesTwice(boxes1, boxes2) {
  boxes1.forEach((box) => console.log(box));
  boxes2.forEach((box) => console.log(box));
}

// is it O(n) ? NO   its => O(a + b )  or  O( x , y ) ..... no matters for the characters
// because boxes can be 5 items, while boxes2 can be maybe 1000000 items so:
// DIFFERENT TERMS FOR INPUTS

// NESTED LOOPS  (   O(n^2)  )
// log all pairs of array
const boxes2 = [1, 2, 3, 4, 5];

function logPairs(array) {
  for (let i = 0; i < array.length; i++) {
    for (let j = 0; j < array.length; j++) {
      console.log(array[i], array[j]);
    }
  }
}
logPairs(boxes2);
// the BigO of it is: O(n * n )  => O(n^2)  O of n to the power two         (for nested loops )
// O(n^2)  is called QUADRATIC TIME,  this O notation increases MULTIPLICATIONAL
// so the each element multiplied by it self,      the complexity of this BigO is:  Horrible
// note: if compressBoxesTwice does nested loops in the two arrays, its BigO will be =>  O(a*b) also

// Rule 4: Drop Non-dominant terms
function printAllNumbersThenAllPairSums(numbers) {
  console.log('these are the numbers:');
  numbers.forEach(function (number) {
    console.log(number);
  });

  console.log('and these are their sums:');
  numbers.forEach(function (firstNumber) {
    numbers.forEach(function (secondNumber) {
      console.log(firstNumber + secondNumber);
    });
  });
}

printAllNumbersThenAllPairSums([1, 2, 3, 4, 5]);
// BigO is: O( n + n^2)  =>  Drop Non-dominant term  that means we care about the most important term
// which is n^2  so it becomes:  O(n^2 )       another example:
// O(x^2 + 3x + 100 + n/2)  => O(x^2)

//////////////////////   Space Complexity
function booo(n) {
  for (let i = 0; i < n.length; i++) console.log('boooooo');
}
booo([1, 2, 3, 4, 5, 6]); // what is the O of this function?
// its O(1)  because we are just defining a variable,

function arrayOfHiTimes(n) {
  let hiArray = [];
  for (let i = 0; i < n; i++) {
    hiArray[i] = 'hi';
  }
  return hiArray;
}
arrayOfHiTimes(8); // what is the space complexity of this function?
// O(n)

// find first and last tweet
const array2 = ['fist tweet ', 'another tweet ', 'and tweet also'];
array[0]; // O(1)
array[array2.length - 1]; // O(1)
// easy and straightforward,  but you should also compare the date of tweets!
const array3 = [
  { tweet: 'fist tweet ', date: 2012 },
  { tweet: 'another tweet ', date: 2015 },
  { tweet: 'and tweet also', date: 2022 },
]; // what is the space complexity of this task ?   O(n^2 )
// because we are going to compare each  verses all the other tweets

'what is the o notation of this string? '.length; // it depends, eg in javascript its O(1)
//but in other languages, maybe this length is a function, so it will be higher that O(1)
