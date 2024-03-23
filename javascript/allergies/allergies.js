export class Allergies {
  static ALLERGENS = [
    "eggs",
    "peanuts",
    "shellfish",
    "strawberries",
    "tomatoes",
    "chocolate",
    "pollen",
    "cats",
  ];

  _list = [];

  constructor(score) {
    this._list = Allergies.ALLERGENS.filter((_, i) => score & (1 << i));
  }

  list() {
    return this._list;
  }

  allergicTo(allergen) {
    return this._list.includes(allergen);
  }
}
