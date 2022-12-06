//  given two strings sx and te return if they equal when both are typed out, any
// hash (#) that appears in the string counts as a backspace.
// for example:   'cb#d'  =>  'cd'  (so the # caused previous char to be deleted)
// so   T: 'ab#c'   and S: 'az#c'   => T: becomes 'ac' , S: becomes 'ac', so they match

//constraints
// 1- what happens when two hashes appear beside each other?
// In this case, we delete the two values before the first hash.eg 'ab##' => ''
// 2-  what happens to hashes when there is no character to remove?
// It deletes nothing then just like backspace would. eg 'a###b' => 'b'
// 3- are two empty strings equal to each other?
// In this case, yes, consider two empty strings as equal  eg:
// 'x#y#z#'  and  'a#'   they become  '' and '' , so they are equal
// 4- does case sensitivity matter ?
// yes it does , 'a' is not equal to 'A'

const string1 = "az#b";
const string2 = "af#b";

const buildString = function (str) {
  const builtArray = [];
  for (let p = 0; p < str.length; p++) {
    if (str[p] !== "#") builtArray.push(str[p]);
    else builtArray.pop();
  }

  return builtArray;
};

const backSpaceCompare = function (s, t) {
  const finalS = buildString(s);
  const finalT = buildString(t);
  if (finalS.length !== finalT.length) return false;
  else {
    for (let p = 0; p < finalS.length; p++) {
      if (finalS[p] !== finalT[p]) return false;
    }
  }
  return true;
};

// optimal solution
const backSpaceImproved = function (s, t) {
  let p1 = s.length - 1;
  let p2 = t.length - 1;
  while (p1 >= 0 || p2 >= 0) {
    if (s[p1] === "#" || t[p2] === "#") {
      if (s[p1] === "#") {
        let backcount = 2;
        while (backcount > 0) {
          p1--;
          backcount--;
          if (s[p1] === "#") backcount = backcount + 2;
        }
      }
      if (t[p2] === "#") {
        let backcount = 2;
        while (backcount > 0) {
          p2--;
          backcount--;
          if (t[p2] === "#") backcount = backcount + 2;
        }
      }
    } else {
      if (s[p1] !== t[p2]) return false;
      else {
        p1--;
        p2--;
      }
    }
  }
  return true;
};
