const fs = require("fs");
const _ = require("lodash");
// const masterInput = fs
//   .readFileSync("./input.txt", "utf-8")
//   .split(",")
//   .map(function(x) {
//     return parseInt(x, 10);
//   });

const masterInput = [
  109,
  1,
  204,
  -1,
  1001,
  100,
  1,
  100,
  1008,
  100,
  16,
  101,
  1006,
  101,
  0,
  99
]; // takes no input and produces a copy of itself as output.
// const masterInput = [1102,34915192,34915192,7,4,7,99,0]// should output a 16-digit number.
// const masterInput = [104,1125899906842624,99]// should output the large number in the middle.

// const sequence = [0, 1, 2, 3, 4];

// const combinations = require("js-combinatorics")
//   .permutation(sequence)
//   .toArray();
// console.log(combinations)

// let isFirstInput = true;
// const out = [];
// for (const combination of combinations) {
//   console.log(combination);
//   const ran = runAmplifiers(masterInput, combination);
//   out.push(ran);
// }

// const sorted = _.orderBy(out, "output", "desc");
// const best = sorted[0];
// console.log(sorted);
// console.log(best);

// function runAmplifiers(masterInput, sequence) {
//   const outputA = run(masterInput, sequence[0], 0);
//   const outputB = run(masterInput, sequence[1], outputA);
//   const outputC = run(masterInput, sequence[2], outputB);
//   const outputD = run(masterInput, sequence[3], outputC);
//   const outputE = run(masterInput, sequence[4], outputD);

//   // console.log(outputE)

//   return {
//     output: outputE,
//     sequence: sequence
//   };
// }

run(masterInput);

function run(input) {
  let running = true;
  let ip = 0;
  let relativeBase = 0;
  let limit = 100;
  let count = 0;
  isFirstInput = true;

  while (running && count < limit) {
  // while (running) {
    count = count + 1;
    let out = processBlock(input, ip, relativeBase);
    input = out.input;
    ip = out.ip + out.increment;
    if (out.hasOwnProperty("relativeBase")) {
      // return out.output;
      relativeBase = out.relativeBase;
    }
  }
  // console.log(input);
}

function processBlock(input, ip, relativeBase) {
  const currentInstruction = input[ip];
  const parameterMode = getParameterMode(currentInstruction);
  console.log(parameterMode);
  let first;
  switch (parameterMode.firstMode) {
    case 0:
      first = input[input[ip + 1]];
      break;
    case 1:
      first = input[ip + 1];
      break;
    case 2:
      first = input[input[ip + 1] + relativeBase];
      break;

    default:
      break;
  }
  // parameterMode.firstMode === 1 ? input[ip + 1] : parameterMode.firstMode === 2 ? input[input[ip + 1+relativeBase]] : input[input[ip + 1]] ;

  // console.log(first);
  // let second =
  //   parameterMode.secondMode === 1 ? input[ip + 2] : input[input[ip + 2]];
  let second
  switch (parameterMode.secondMode) {
    case 0:
      second = input[input[ip + 2]];
      break;
    case 1:
      second = input[ip + 2];
      break;
    case 2:
      second = input[input[ip + 2] + relativeBase];
      break;

    default:
      break;
  }
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
      // const userInput = isFirstInput ? sequenceValue : inputValue;
      console.log(`userinput assumed 0`);
      const userInput = 0;
      isFirstInput = false;
      input[destination] = parseInt(userInput, 10);
      return { input, increment: 2, ip };
    case 4:
      // console.log(`output: ${first}`);
      console.log(first);
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
    case 9:
      // console.log(`relativebase: ${relativeBase} ${first}`);
      relativeBase = relativeBase + first;
      // if (first === second) {
      //   input[input[ip + 3]] = 1;
      // } else {
      //   input[input[ip + 3]] = 0;
      // }
      // return { input, increment: 4, ip };
      return { input, increment: 2, ip, relativeBase };
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
