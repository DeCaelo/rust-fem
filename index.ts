function multiply(num: number | undefined): number {
  return (num ?? 0) * 5;
}

console.log(multiply(undefined));
console.log(multiply(10));
