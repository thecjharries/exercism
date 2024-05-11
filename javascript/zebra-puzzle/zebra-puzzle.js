//
// This is only a SKELETON file for the 'Zebra Puzzle' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

const permutateArray = (array) => {
  const result = [];
  const permute = (arr, m = []) => {
    if (arr.length === 0) {
      result.push(m);
    } else {
      for (let i = 0; i < arr.length; i++) {
        const curr = arr.slice();
        const next = curr.splice(i, 1);
        permute(curr.slice(), m.concat(next));
      }
    }
  };
  permute(array);
  return result;
};

const rightOf = (a, b) => {
  return a === b + 1;
};

const nextTo = (a, b) => {
  return rightOf(a, b) || rightOf(b, a);
};

const PERMUTATIONS = permutateArray([1, 2, 3, 4, 5]);
const COLORS = ["red", "green", "ivory", "yellow", "blue"];
const NATIONALITIES = [
  "english",
  "spaniard",
  "ukrainian",
  "norwegian",
  "japanese",
];
const BEVERAGES = ["coffee", "tea", "milk", "orangeJuice", "water"];
const CIGARETTES = [
  "oldGold",
  "kools",
  "chesterfields",
  "luckyStrike",
  "parliaments",
];
const PETS = ["dog", "snails", "fox", "horse", "zebra"];

export class ZebraPuzzle {
  constructor() {
    this.personDrinkingWater = null;
    this.personOwningZebra = null;
  }

  waterDrinker() {
    if (!this.personDrinkingWater) {
      this.solve();
    }
    return this.personDrinkingWater;
  }

  zebraOwner() {
    if (!this.personOwningZebra) {
      this.solve();
    }
    return this.personOwningZebra;
  }

  solve() {
    for (const permutation of PERMUTATIONS) {
      if (this.solveColor(permutation)) {
        return;
      }
    }
  }

  solveColor(permutation) {
    COLORS.forEach((color, index) => (this[color] = permutation[index]));
    if (rightOf(this.green, this.ivory)) {
      for (const newPerm of PERMUTATIONS) {
        if (this.solveNationalities(newPerm)) {
          return true;
        }
      }
    }
    return false;
  }

  solveNationalities(permutation) {
    NATIONALITIES.forEach(
      (nationality, index) => (this[nationality] = permutation[index])
    );
    if (
      this.english === this.red &&
      this.norwegian === 1 &&
      nextTo(this.norwegian, this.blue)
    ) {
      this.nationalities = [];
      this.nationalities[this.english] = "EnglishMan";
      this.nationalities[this.spaniard] = "Spaniard";
      this.nationalities[this.ukrainian] = "Ukrainian";
      this.nationalities[this.norwegian] = "Norwegian";
      this.nationalities[this.japanese] = "Japanese";
      for (const newPerm of PERMUTATIONS) {
        if (this.solveBeverages(newPerm)) {
          return true;
        }
      }
    }
    return false;
  }

  solveBeverages(permutation) {
    BEVERAGES.forEach(
      (beverage, index) => (this[beverage] = permutation[index])
    );
    if (
      this.coffee === this.green &&
      this.ukrainian === this.tea &&
      this.milk === 3
    ) {
      for (const newPerm of PERMUTATIONS) {
        if (this.solveCigarettes(newPerm)) {
          return true;
        }
      }
    }
    return false;
  }

  solveCigarettes(permutation) {
    CIGARETTES.forEach(
      (cigarette, index) => (this[cigarette] = permutation[index])
    );
    if (
      this.kools === this.yellow &&
      this.luckyStrike === this.orangeJuice &&
      this.parliaments === this.japanese
    ) {
      for (const newPerm of PERMUTATIONS) {
        if (this.solvePets(newPerm)) {
          return true;
        }
      }
    }
    return false;
  }

  solvePets(permutation) {
    PETS.forEach((pet, index) => (this[pet] = permutation[index]));
    if (
      this.spaniard === this.dog &&
      this.oldGold === this.snails &&
      nextTo(this.chesterfields, this.fox) &&
      nextTo(this.kools, this.horse)
    ) {
      this.personDrinkingWater = this.nationalities[this.water];
      this.personOwningZebra = this.nationalities[this.zebra];
      return true;
    }
    return false;
  }
}
