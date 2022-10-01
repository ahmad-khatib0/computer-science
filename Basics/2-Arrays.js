const strings = ["a", "b", "c", "d", "e"]; // 5*5 = 25 bytes

strings[1]; // super fast, because we're telling js FROM where to grab this element exactly in RAM
strings.push("f"); // O(1) , just insert at the end, very fast also
strings.pop(); // O(1)

strings.unshift("h"); // O(n) , because when inserting, after that its need to reorder the indexes of the array ,
/// so it will shift them to the right , and reassign all the elements indexes

strings.splice(2, 0, "n"); // O(n/2) because its a little bit more efficient than unshift, because js
// now has to reassign the indexes to new indexes ONLY after the 3th element where we added the new element,
// note: O(n/2)  simplified to O(n)

//////// there are two types of arrays, STATIC VS DYNAMIC, , e,g in C++ we have a static array:
// int myNum[3] = {10, 20, 30};  this array has fixed number of element, e,g if you wanna have a bigger
//array, we would garbage this array, and move it entirely to another  location in RAM ,
// although when working with dynamic arrays as in js we don't have this headache, but when we push
// to an array in js , behind the scene, it destroy the previous one, and creates another one,
// with the new number of elements,  so this is why for example C++ is more efficient than js

/////////////////  IMPLEMENTING AN ARRAY  (our own data structure)  /////////////////
class MyArray {
  constructor() {
    this.length = 0;
    this.data = {};
  }
  get(index) {
    return this.data[index];
  }
  push(item) {
    this.data[this.length] = item;
    this.length++;
    return this.length;
  }
  pop() {
    const lastItem = this.data[this.length - 1];
    delete this.data[this.length - 1];
    this.length--;
    return lastItem;
  }
  delete(index) {
    const item = this.data[index];
    this.shiftItems(index);
    return item;
  }
  shiftItems(index) {
    for (let i = index; i < this.length - 1; i++) {
      // start on the index where deleting occurred
      this.data[i] = this.data[i + 1]; //move what is the i on, to what is the next of it
    }
    delete this.data[this.length - 1]; // delete the last item,, if we don't do that, the last item will be kept
    // because in the loop, we had shifted all items to the right , that last item we didn't touch it,
    this.length--;
  }
}

const newArray = new MyArray();
newArray.push("hi");
newArray.push("you");
newArray.push("!");
// newArray.pop();
newArray.delete(1);
console.log(newArray);

///////////////////
//given an array of integer, return the indices of the two numbers that add up to a given target
const numsArray = [1, 3, 7, 9, 2];

const findTwoSum = function (nums, target) {
  for (let p1 = 0; p1 < nums.length; p1++) {
    const numberToFind = target - nums[p1];
    for (let p2 = p1 + 1; p2 < nums.length; p2++) {
      if (numberToFind === nums[p2]) {
        return [p1, p2];
      }
    }
  }

  return null;
};

findTwoSum(numsArray, 11);

//  optimal solution  with HASH MAP
const findTwoSum = function (nums, target) {
  const numsMap = {};
  for (let p = 0; p < nums.length; p++) {
    const currentMapVal = numsMap[nums[p]];
    if (currentMapVal >= 0) {
      return [currentMapVal, p];
    } else {
      const numberToFind = target - nums[p];
      numsMap[numberToFind] = p;
    }
  }
  return null;
};

console.log(findTwoSum(numsArray, targetToFind));
