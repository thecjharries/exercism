export class InvalidInputError extends Error {
  constructor(message: string) {
    super()
    this.message = message || 'Invalid Input'
  }
}

type Direction = 'north' | 'east' | 'south' | 'west'
type Coordinates = [number, number]

export class Robot {
    public coords: Coordinates = [0, 0]
    public direction: Direction = 'north'

  get bearing(): Direction {
    return this.direction
  }

  get coordinates(): Coordinates {
    return this.coords
  }

  place(args: { x: number; y: number; direction: string }) {
    this.coords = [args.x, args.y]
    if (['north', 'east', 'south', 'west'].includes(args.direction)) {
      this.direction = args.direction as Direction
    } else {
        throw new InvalidInputError('Invalid Input')
    }
  }

    turnRight() {
        const directions: Direction[] = ['north', 'east', 'south', 'west']
        const index = directions.indexOf(this.direction)
        this.direction = directions[(index + 1) % 4]
    }

    turnLeft() {
        const directions: Direction[] = ['north', 'west', 'south', 'east']
        const index = directions.indexOf(this.direction)
        this.direction = directions[(index + 1) % 4]
    }

    advance() {
        switch (this.direction) {
            case 'north':
                this.coords[1]++
                break
            case 'east':
                this.coords[0]++
                break
            case 'south':
                this.coords[1]--
                break
            case 'west':
                this.coords[0]--
                break
        }
    }

  evaluate(instructions: string) {
    instructions.split('').forEach((instruction) => {
      switch (instruction) {
        case 'L':
          this.turnLeft()
          break
        case 'R':
          this.turnRight()
          break
        case 'A':
          this.advance()
          break
        default:
          throw new InvalidInputError('Invalid Input')
      }
    })
  }
}
