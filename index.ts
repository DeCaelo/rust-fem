import fs from 'fs';

const fileName = process.argv[2];

fs.readFileSync(fileName)
  .toString()
  .split('\n')
  .forEach((line) => console.log(line));
