# Create a new score board with an initial entry.
#   input: none.
#   output: a score board object with a default player and score.
def create_score_board:
  {"The Best Ever": 1000000};

# Add a player to a score board.
#   input: a score board object.
#   output: the score board with the new player added.
def add_player(player; score):
  .[player] = score;

# Remove a player from a score board.
#   input: a score board object.
#   output: the score board with the player removed, if they exist.
def remove_player(player):
  del(.[player]);

# Increase a player's score by the given amount.
#   input: a score board object.
#   output: the score board with the player's score increased, if they exist.
def update_score(player; points):
  halt_error("Please implement the update_score function");

# Apply 100 bonus points to all players on the board.
#   input: a score board object.
#   output: the score board with each player's score increased.
def apply_monday_bonus:
  halt_error("Please implement the apply_monday_bonus function");

# Calculate the total score of all players.
#   input: a score board object.
#   output: the sum of all scores, or zero for an empty board.
def total_score:
  halt_error("Please implement the total_score function");
