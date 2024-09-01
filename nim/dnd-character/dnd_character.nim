import std/random, math

type
  Character* = object
    strength*: int
    dexterity*: int
    constitution*: int
    intelligence*: int
    wisdom*: int
    charisma*: int
    hitpoints*: int

proc ability*: int =
  var rolls: array[4, int]
  for roll in rolls.mitems:
    roll = rand(1..6)
  rolls.sum - rolls.min

proc modifier*(n: int): int =
  floor((n - 10) / 2).int

proc initCharacter*: Character =
  for field in result.fields:
    field = ability()
  result.hitpoints = 10 + modifier(result.constitution)
