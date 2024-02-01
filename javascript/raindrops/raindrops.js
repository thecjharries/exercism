//
// This is only a SKELETON file for the 'Raindrops' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const convert = (number) => {
  let result = "";
  if (0 === number % 3) {
    result += "Pling";
  }
  if (0 === number % 5) {
    result += "Plang";
  }
  if (0 === number % 7) {
    result += "Plong";
  }
  if ("" === result) {
    result = number.toString();
  }
  return result;
};
