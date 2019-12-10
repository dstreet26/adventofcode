const fs = require("fs");
const readlineSync = require("readline-sync");

let running = true;

let input = fs
  .readFileSync("./input.txt", "utf-8")
  .split(",")
  .map(function(x) {
    return parseInt(x, 10);
  });

// let input = [1002,4,3,4,33];
// let input = [1101,100,-1,4,0];

// let input = [3,9,8,9,10,9,4,9,99,-1,8]
// let input = [3,9,7,9,10,9,4,9,99,-1,8]
// let input = [3,3,1108,-1,8,3,4,3,99]
// let input = [3,3,1107,-1,8,3,4,3,99]
// let input = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]
// let input = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1]

let ip = 0;

while (running) {
  let out = processBlock(input, ip);
  input = out.input;
  ip = out.ip + out.increment;
}

function processBlock(input, ip) {
  count += 1;
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
      var userInput = readlineSync.question("UserInput: ");
      input[destination] = parseInt(userInput, 10);
      return { input, increment: 2, ip };
    case 4:
      console.log(first);
      return { input, increment: 2, ip };
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
