import fs from 'fs';

const fileName = process.argv[2];

fs.readFileSync(fileName)
  .toString()
  .split('\n')
  .forEach((line) => {
    const print = parseInt(line);
    if (isNaN(print)) {
      console.log('Line is not a number');
    }
    console.log(print);
  });
