// given an array of integers representing an elevation map where the width of each
// bar is 1, return how much rainwater can be trapped?
// CONSTRAINTS 
// 1- do the left and right sides of the graph count as walls?
//  No, the sides are not walls 
// 2- will there be negative integers?
// no. Assume all integers are positive.

const elevationArray = [0, 1, 0, 2, 1, 0, 3, 1, 0, 1, 2]

const getTrappedRainwater = function(heights) {
  let totalWater = 0;
  for(let p = 0; p < heights.length; p++) {
    let leftP = p, rightP = p, maxLeft = 0, maxRight = 0;

    while(leftP >= 0) {
      maxLeft = Math.max(maxLeft, heights[leftP]);
      leftP--; // -- to move to the left side
    }
    while(rightP < heights.length) {
      maxRight = Math.max(maxRight, heights[rightP]);
      rightP++; // ++ to move to the right side
    }
    
    const currentWater = Math.min(maxLeft, maxRight) - heights[p];
    if(currentWater >= 0)  totalWater += currentWater 
  }

  return totalWater;
}
console.log(getTrappedRainwater(elevationArray));

// optimal solution 
const getTrappedRainwaterImproved  =  function(heights) { 
  let left = 0 ; 
  let right = heights.length - 1; 
  let maxLeft =0 ; 
  let maxRight = 0 ; 
  let total = 0 ;

  while( left < right) { 
    if(heights[left] <= heights[right]) { 
      if(heights[left] >=  maxLeft) maxLeft  = heights[left]; //update wall, because its taller than previous one
      else total += maxLeft - heights[left]  //update water, because maxLeft is taller than currentHeight, 
      // so there is like a hole, which will store or held a block or blocks of water 
      left++; //move right 
    } else { 
      if(heights[right] >= maxRight) maxRight = heights[right] ; 
      else total += maxRight - heights[right];
      right--;
    }
  }
  return total;
}
