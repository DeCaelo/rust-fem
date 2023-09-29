type Custom = {
  age: number;
  name: string;
};

type Item = number | string | Custom;

function append(items: Item[]) {
  items.push('hello fem!');
}

const items: Item[] = [];

append(items);
console.log(items);

const numbers: number[] = [];

append(numbers);
console.log(items);
