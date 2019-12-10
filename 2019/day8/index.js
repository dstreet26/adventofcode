const fs = require("fs");
const _ = require("lodash");
const input = fs
  .readFileSync("./input.txt", "utf-8")
  .split("")
  .map(function(x) {
    return parseInt(x, 10);
  });

//  25 pixels wide and 6 pixels tall.
const width = 25;
const height = 6;

const layers = _.chunk(input, width * height);

// layer that contains the fewest 0 digits.
// On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
const out = [];
for (const layer of layers) {
  console.log(layer);
  out.push({
    zeroes: _.filter(layer, d => d === 0).length,
    ones: _.filter(layer, d => d === 1).length,
    twos: _.filter(layer, d => d === 2).length
  });
}

console.log(out);
const sorted = _.orderBy(out, "zeroes");
const best = sorted[0];
console.log(sorted);
console.log(best);

console.log(best.ones * best.twos);
