# Globals for the directions
# Change the values as you see fit
EAST = (1, 0)
NORTH = (0, 1)
WEST = (-1, 0)
SOUTH = (0, -1)
DIRECTIONS = [NORTH, EAST, SOUTH, WEST]


class Robot:
    def __init__(self, direction=NORTH, x_pos=0, y_pos=0):
        self.direction = direction
        self.x = x_pos
        self.y = y_pos

    @property
    def coordinates(self):
        return (self.x, self.y)

    def turn_right(self):
        index = DIRECTIONS.index(self.direction)
        self.direction = DIRECTIONS[(index + 1) % 4]

    def turn_left(self):
        index = DIRECTIONS.index(self.direction)
        self.direction = DIRECTIONS[(index - 1) % 4]

    def advance(self):
        self.x += self.direction[0]
        self.y += self.direction[1]

    def move(self, moves=""):
        entries = list(moves)
        for entry in entries:
            match entry:
                case 'R':
                    self.turn_right()
                case 'L':
                    self.turn_left()
                case 'A':
                    self.advance()
