# We will get more in-depth about jq functions in a later lesson.
# For now, your job is to implement the logic of the tasks in the function
# bodies, so that they output the correct values.

# We haven't covered regular expressions yet.
# This function will remove leading and trailing whitespace
# from the input string.
def trim: sub("^\\s+"; "") | sub("\\s+$"; "");

# Task 1. Get message from a log line
def message:
  sub("^\\[\\w+\\]:\\s+"; "") | trim # implement the body of this function
;

# Task 2. Get log level from a log line
def log_level:
  match("^.*\\[(\\w+)\\].*$").captures[0].string | ascii_downcase # implement the body of this function
;

# Task 3. Reformat a log line
def reformat:
  . # implement the body of this function
;
