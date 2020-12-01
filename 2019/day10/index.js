const fs = require("fs");

test(__dirname + "/test1.txt", 3, 4, 8);
// test('test2.txt', 5, 8, 33)
// test('test3.txt', 1, 2, 35)
// test('test4.txt', 6, 3, 41)
// test('test5.txt', 11, 13, 210)
// run('input.txt')

function test(fileName, x, y, count) {
  // console.log(`testing ${fileName}...`)
  const processed = run(fileName);
  console.log(processed);
  if (processed.x === x && processed.y === y && processed.count === count) {
    console.log("success");
    return true;
  } else {
    console.log(`failure`);
    console.log(`expected: ${x} ${y} ${count}`);
    console.log(`got: ${processed.x} ${processed.y} ${processed.count}`);
    return false;
  }
}

function read(fileName) {
  const contents =  fs.readFileSync(fileName, "utf-8").split("\n");
  return { contents, height: contents.length, width: contents[0].length}
}

function run(fileName) {
  const asteroidMap = read(fileName);
  console.log(asteroidMap);
  const x = 0
  const y = 0
  const count = 0
  for (let i = 0; i < asteroidMap.width; i++) {
      for (let j = 0; j < asteroidMap.height; j++) {
          const element = asteroidMap.contents[i][j];
          element
          
      }
    //   const element = width[i];

      
  }

  return { x,y, count};
}
