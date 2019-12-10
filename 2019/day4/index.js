const _ = require("lodash");
// let min = 112233
// let min = 111122
// let min = 123444
let min = 122224;
// let min = 158126;
let max = 624574;
// It is a six-digit number.
// The value is within the range given in your puzzle input.
// Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
// Two adjacent digits are the same (like 22 in 122345).

let counter = 0;

for (let i = min; i < min + 1; i++) {
  const digits = ("" + i).split("");
  if (isDecreasing(digits)) {
    continue;
  }

  let consecutiveCounter = 0;
  let rejectIt = false;
  for (let index = 0; index < digits.length; index++) {
    if (digits[index] === digits[index + 1]) {
      consecutiveCounter++;
    } else {
      if (isEven(consecutiveCounter) && consecutiveCounter > 0) {
        rejectIt = true;
      }
      consecutiveCounter = 0;
    }
  }
  if (!rejectIt) {
    counter++;
  }
}
function isEven(n) {
  return n % 2 == 0;
}
console.log(counter);

function isDecreasing(digits) {
  for (let index = 0; index < digits.length; index++) {
    if (digits[index] > digits[index + 1]) {
      return true;
    }
  }
  return false;
}
