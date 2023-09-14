# We will get more in-depth about jq functions in a later lesson.
# A function begins with "def", ends with ";", and consists of a
# single pipeline just like a "main" jq program.
#
# For now, your job is to implement the logic of the tasks in the function
# bodies, so that they output the correct values.

# Task 1: calculate the production rate per hour
def production_rate_per_hour:
  # remove the constant and implement this function
  if 0 < . && . <= 4 then
    1
  elif 4 < . && . <= 8 then
    .9
  elif 8 < . && . <= 9 then
    .8
  elif 9 < . && . <= 10 then
    .77
  else
    0
  end
;

# Task 2: calculate the number of working items produces per minute
def working_items_per_minute:
  # remove the constant and implement this function
  if 0 < . and . <= 4 then
    221
  elif 4 < . and . <= 8 then
    199
  elif 8 < . and . <= 9 then
    170
  elif 9 < . and . <= 10 then
    153
  else
    0
  end
;


# Please don't change the line below: it is responsible for passing
# the input speed value (a number between 0 and 10 inclusive)
# to the two functions defined above.
#
.speed | (production_rate_per_hour, working_items_per_minute)
