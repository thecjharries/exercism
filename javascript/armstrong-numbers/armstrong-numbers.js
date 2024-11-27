export const isArmstrongNumber = (input) => {
  const digits = [];
  let number = input;
  while (number > 0) {
    digits.push(number % 10);
    number = Math.floor(number / 10);
  }
  const power = digits.length;
  return input === digits.reduce((acc, digit) => acc + digit ** power, 0);
};
