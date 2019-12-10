const desired = 19690720;

let running = true;

for (let noun = 0; noun <= 99; noun++) {
  for (let verb = 0; verb <= 99; verb++) {
    // prettier-ignore
    let input = [1, noun, verb, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 2, 19, 6, 23, 2, 13, 23, 27, 1, 9, 27, 31, 2, 31, 9, 35, 1, 6, 35, 39, 2, 10, 39, 43, 1, 5, 43, 47, 1, 5, 47, 51, 2, 51, 6, 55, 2, 10, 55, 59, 1, 59, 9, 63, 2, 13, 63, 67, 1, 10, 67, 71, 1, 71, 5, 75, 1, 75, 6, 79, 1, 10, 79, 83, 1, 5, 83, 87, 1, 5, 87, 91, 2, 91, 6, 95, 2, 6, 95, 99, 2, 10, 99, 103, 1, 103, 5, 107, 1, 2, 107, 111, 1, 6, 111, 0, 99, 2, 14, 0, 0];

    let ip = 0;
    running = true;

    while (running) {
      input = processBlock(
        input[ip],
        input[ip + 1],
        input[ip + 2],
        input[ip + 3],
        input
      );
      ip = ip + 4;
    }
    if (input[0] === desired) {
      console.log(input[0]);
      console.log(`noun: ${noun}`);
      console.log(`verb: ${verb}`);
      console.log(`answer: ${100 * noun + verb}`);
    }
  }
}

function processBlock(opcode, one, two, store, input) {
  switch (opcode) {
    case 1:
      input[store] = input[one] + input[two];
      return input;

    case 2:
      input[store] = input[one] * input[two];
      return input;
    case 99:
      running = false;
      return input;

    default:
      console.log("!!");
      return input;
  }
}
