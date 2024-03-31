-include ../global.mk

# Aliases for executables
GIT ?= git
MIX ?= mix
RM ?= rm
SED ?= sed

# Exercism variables
SUBMISSIONS = lib/*.ex
TRACK = elixir

boot-tests::
	$(SED) -i '/@tag :pending/d' $(EXERCISE)/test/*_test.exs
	$(GIT) add $(EXERCISE)/test/*_test.exs
	$(GIT) commit -m "Enable all tests"

test::
	$(MIX) test

clean::
	$(RM) -rf _build
