-include ../global.mk

# Aliases for executables
GIT ?= git
MIX ?= mix
RM ?= rm
SED ?= sed

# Exercism variables
SUBMISSIONS = lib/$(EXERCISE_SNAKE_CASE).ex
TRACK = elixir

boot-tests::
	$(SED) -i '/@tag :pending/d' test/$(EXERCISE_SNAKE_CASE)_test.exs
	$(GIT) add test/$(EXERCISE_SNAKE_CASE)_test.exs
	$(GIT) commit -m "Enable all tests"

test::
	$(MIX) test

clean::
	$(RM) -rf _build
