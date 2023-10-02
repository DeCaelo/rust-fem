function multiply(nums: number[], index: number): number {
  return (nums[index] ?? index) * 5;
}

console.log(multiply([10, 9, 8], 1));
