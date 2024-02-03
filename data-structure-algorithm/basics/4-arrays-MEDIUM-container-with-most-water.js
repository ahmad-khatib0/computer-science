// You are given an array of positive integers where each integer represents the height of a vertical line on a chart.
// Find two lines which together with the x axis, forms a container that would hold the greatest amounts
// of water. Return the area of water it would hold.

// constraints
// 1- does the thickness of the lines affect the area?
// no, we can assume they take up no space

//2- do the left and right size of the graph count as walls?
// The sides cannot be used to form a container.

//3- does a higher line inside our container affect our area?
// No lines inside a container don't affect the area.

const array = [7, 1, 2, 3, 9];

let mostWater = function (arr) {
  let max = 0;
  for (let a = 0; a < arr.length; a++) {
    for (let b = a + 1; b < arr.length; b++) {
      const cal = Math.min(arr[a], arr[b]) * (b - a);
      if (cal > max) max = cal;
    }
    //another structure
    // for(let p2 = p1 + 1; p2 < heights.length; p2++) {
    //   const height = Math.min(heights[p1], heights[p2]);
    //   const width = p2 - p1;
    //   const area = height * width;
    //   maxArea = Math.max(maxArea, area);
    // }
  }
  return max;
};

// optimal solution with two pointer technich

const heightsArray = [4, 8, 1, 2, 3, 9, 4, 3];
const getMaxWaterContainer = function (heights) {
  let p1 = 0,
    p2 = heights.length - 1,
    maxArea = 0;

  // at the first iteration of the while loop, we'll begin from the right to the left , only a head of time, and go to
  // the left side,
  while (p1 < p2) {
    const height = Math.min(heights[p1], heights[p2]);
    const width = p2 - p1;
    const area = height * width;
    maxArea = Math.max(maxArea, area);

    // const compare = `${heights[p1]} ${heights[p2]}`;
    // console.log({ height, width, maxArea, compare });

    if (heights[p1] <= heights[p2]) {
      p1++;
      console.log("inc", p1, p2);
      // the most right number is HIGHER than the current left number that is we are doing the iteration over now,,
      // so move the the current left number that is we are doing the iteration over now by one position to the right
      // and check the remianing possibilites
    } else {
      p2--;
      console.log("dec", p1, p2);
      // the most right number is LOWER than the current left number that is we are doing the iteration over now,,
      // so move thet last right position element to the left by one position,because its not the highest and we are
      // looking for the highest number, and check the remianing possibilites
    }
  }
  return maxArea;
};
console.log(getMaxWaterContainer(heightsArray));
