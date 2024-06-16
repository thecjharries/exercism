from collections import Counter


def four_of_a_kind(dice):
    for i in dice:
        if dice.count(i) >= 4:
            return i * 4
    return 0


YACHT = (lambda dice: 50 if len(set(dice)) == 1 else 0)
ONES = (lambda dice: sum([1 for i in dice if i == 1]))
TWOS = (lambda dice: sum([2 for i in dice if i == 2]))
THREES = (lambda dice: sum([3 for i in dice if i == 3]))
FOURS = (lambda dice: sum([4 for i in dice if i == 4]))
FIVES = (lambda dice: sum([5 for i in dice if i == 5]))
SIXES = (lambda dice: sum([6 for i in dice if i == 6]))
FULL_HOUSE = (lambda dice: sum(dice) if sorted(
    Counter(dice).values()) == [2, 3] else 0)
FOUR_OF_A_KIND = four_of_a_kind
LITTLE_STRAIGHT = (lambda dice: 30 if sorted(dice) == [1, 2, 3, 4, 5] else 0)
BIG_STRAIGHT = (lambda dice: 30 if sorted(dice) == [2, 3, 4, 5, 6] else 0)
CHOICE = sum


def score(dice, category):
    return category(dice)
