const fs = require("fs");
const _ = require("lodash");

const input = fs.readFileSync("./input.txt", "utf-8").split("\n");
// const input = ['COM)B','B)C','C)D','D)E','E)F','B)G','G)H','D)I','E)J','J)K','K)L']
// const input = [
//   "COM)B",
//   "B)C",
//   "C)D",
//   "D)E",
//   "E)F",
//   "B)G",
//   "G)H",
//   "D)I",
//   "E)J",
//   "J)K",
//   "K)L",
//   "K)YOU",
//   "I)SAN"
// ];

let tree = {};
let planets = new Set();
let counter = 0;
for (const line of input) {
  const splitted = line.split(")");
  const left = splitted[0];
  const right = splitted[1];
  planets.add(left);
  planets.add(right);
  tree[right] = left;
}
for (const planet of planets) {
  counter = counter + numOrbits(planet);
}

console.log(`Number of total bodies: ${planets.size}`);
console.log(`Number of total orbits: ${counter}`);

const numTransfers = getTransfers("YOU", "SAN");
console.log(`Number of Transfers: ${numTransfers}`);

function getTransfers(from, dest) {
  const fromAncestors = getAncestors(from);
  const destAncestors = getAncestors(dest);
  const commonAncestor = getCommonAncestor(fromAncestors, destAncestors);
  for (let i = 0; i < fromAncestors.length; i++) {
    if (fromAncestors[i] === commonAncestor) {
      for (let j = 0; j < destAncestors.length; j++) {
        if (destAncestors[j] === commonAncestor) {
          return i + j - 2;
        }
      }
    }
  }
}
function getCommonAncestor(fromAncestors, destAncestors) {
  const intersection = _.intersection(fromAncestors, destAncestors);
  return intersection[0];
}

function getAncestors(key) {
  const out = [];
  while (true) {
    if (tree.hasOwnProperty(key)) {
      out.push(key);
      key = tree[key];
    } else {
      return out;
    }
  }
}

function numOrbits(planet) {
  if (tree.hasOwnProperty(planet)) {
    return 1 + numOrbits(tree[planet]);
  } else {
    return 0;
  }
}
