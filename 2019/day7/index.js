const fs = require("fs");
const _ = require("lodash");
const masterInput = fs
  .readFileSync("./input.txt", "utf-8")
  .split(",")
  .map(function(x) {
    return parseInt(x, 10);
  });
const sequence = [0, 1, 2, 3, 4];

const combinations = require("js-combinatorics")
  .permutation(sequence)
  .toArray();
// console.log(combinations)

let isFirstInput = true;
const out = [];
for (const combination of combinations) {
  console.log(combination);
  const ran = runAmplifiers(masterInput, combination);
  out.push(ran);
}

const sorted = _.orderBy(out, "output", "desc");
const best = sorted[0];
console.log(sorted);
console.log(best);

function runAmplifiers(masterInput, sequence) {
  const outputA = run(masterInput, sequence[0], 0);
  const outputB = run(masterInput, sequence[1], outputA);
  const outputC = run(masterInput, sequence[2], outputB);
  const outputD = run(masterInput, sequence[3], outputC);
  const outputE = run(masterInput, sequence[4], outputD);

  // console.log(outputE)

  return {
    output: outputE,
    sequence: sequence
  };
}

function run(input, sequenceValue, inputValue) {
  let running = true;
  let ip = 0;
  isFirstInput = true;

  while (running) {
    let out = processBlock(input, ip, sequenceValue, inputValue);
    input = out.input;
    ip = out.ip + out.increment;
    if (out.hasOwnProperty("output")) {
      return out.output;
    }
  }
}

function processBlock(input, ip, sequenceValue, inputValue) {
  const currentInstruction = input[ip];
  const parameterMode = getParameterMode(currentInstruction);
  let first =
    parameterMode.firstMode === 1 ? input[ip + 1] : input[input[ip + 1]];
  let second =
    parameterMode.secondMode === 1 ? input[ip + 2] : input[input[ip + 2]];
  destination = null;
  switch (parameterMode.opcode) {
    case 1:
      destination = input[ip + 3];
      input[destination] = first + second;
      return { input, increment: 4, ip };

    case 2:
      destination = input[ip + 3];
      input[destination] = first * second;
      return { input, increment: 4, ip };
    case 3:
      destination = input[ip + 1];
      const userInput = isFirstInput ? sequenceValue : inputValue;
      isFirstInput = false;
      input[destination] = parseInt(userInput, 10);
      return { input, increment: 2, ip };
    case 4:
      // console.log(`output: ${first}`);
      return { input, increment: 2, ip, output: first };
    case 5:
      if (first !== 0) {
        ip = second;
        return { input, increment: 0, ip };
      } else {
        return { input, increment: 3, ip };
      }
    case 6:
      // console.log(first, second);
      if (first === 0) {
        // console.log(`iszero. current ip: ${ip}, new ip ${second}`)
        ip = second;
        return { input, increment: 0, ip };
      } else {
        // console.log(`not zero. current ip: ${ip}, new ip should be ${ip+2}`)
        return { input, increment: 3, ip };
      }
    case 7:
      if (first < second) {
        input[input[ip + 3]] = 1;
      } else {
        input[input[ip + 3]] = 0;
      }
      return { input, increment: 4, ip };
    case 8:
      if (first === second) {
        input[input[ip + 3]] = 1;
      } else {
        input[input[ip + 3]] = 0;
      }
      return { input, increment: 4, ip };
    case 99:
      running = false;
      console.log("Halting");
      return { input };

    default:
      console.log(`unknown opcode ${parameterMode.opcode}`);
      process.exit(1);
      running = false;
      return { input };
  }
}

function getParameterMode(input) {
  const ones = Math.floor(input % 10) || 0;
  const tens = Math.floor((input / 10) % 10) || 0;
  return {
    opcode: parseInt(tens.toString() + ones.toString(), 10),
    firstMode: Math.floor((input / 100) % 10) || 0,
    secondMode: Math.floor((input / 1000) % 10) || 0,
    thirdMode: Math.floor((input / 10000) % 10) || 0
  };
}
