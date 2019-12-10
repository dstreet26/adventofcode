const fs = require('fs');
const _ = require("lodash");

// const firstLine = ["R8", "U5", "L5", "D3"];
// const secondLine = ["U7", "R6", "D4", "L4"];

// const firstLine = ["R75","D30","R83","U83","L12","D49","R71","U7","L72"]
// const secondLine = ["U62","R66","U55","R34","D71","R55","D58","R83"]

// const firstLine = ["R98","U47","R26","D63","R33","U87","L62","D20","R33","U53","R51"]
// const secondLine = ["U98","R91","D20","R16","D67","R40","U7","R15","U6","R7"]

const input = fs.readFileSync('input.txt','utf-8').split('\n')
const firstLine = input[0].split(',')
const secondLine = input[1].split(',')

// console.log(firstLine)
// console.log(secondLine)

const firstSet = run(firstLine);
const secondSet = run(secondLine);

function run(theLine) {
  const out = [];

  let currentX = 0;
  let currentY = 0;
  let counter = 1;

  theLine.forEach(element => {
    const direction = element[0];
    const amount = element.slice(1);
    for (let i = 0; i < amount; i++) {
      switch (direction) {
        case "U":
          currentY += 1;
          break;
        case "D":
          currentY -= 1;
          break;
        case "L":
          currentX -= 1;
          break;
        case "R":
          currentX += 1;
          break;
        default:
          break;
      }
      out.push({ x: currentX, y: currentY, cost: counter });
      counter++;
    }
  });

  return out;
}

// let intersections = new Set([...firstSet].filter(x => secondSet.has(x)));
// console.log(firstSet.entries()[0])

// let intersections = _.intersectionWith(firstSet, secondSet, function(a,b){
//   return a.x  === b.x && a.y === b.y;
// });
const intersections = [];
for (const firstPoint of firstSet) {
  for (const secondPoint of secondSet) {
    if (firstPoint.x === secondPoint.x && firstPoint.y === secondPoint.y) {
      intersections.push({
        x: firstPoint.x,
        y: firstPoint.y,
        cost: firstPoint.cost + secondPoint.cost
      });
    }
  }
}
console.log(intersections);
console.log(firstSet.length)
console.log(secondSet.length)

const out = [];
for (const point of intersections) {
  const cost = Math.abs(parseInt(point.x, 0)) + Math.abs(parseInt(point.y, 0));
  console.log(cost)
  out.push({
    point,
    cost
  });
}

const sorted = _.sortBy(out, "cost");
console.log(out);
console.log(sorted[0]);
console.log(sorted[0].point.cost);