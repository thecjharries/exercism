-include ../global.mk

# Aliases for executables
CLJ ?= clojure
RM ?= rm

# Exercism variables
SUBMISSIONS = src/$(EXERCISE_SNAKE_CASE).clj
TRACK = clojure

clean::
	$(RM) -rf .cpcache

test::
	$(CLJ) -X:test
