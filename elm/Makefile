-include ../global.mk

# Aliases for executables
# CP ?= cp
ELM_TEST ?= elm-test
# GH ?= gh
# GIT ?= git
RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = src/$(EXERCISE_PASCAL_CASE).elm
TRACK = elm

boot-tests::
	$(SED) -i 's/skip <|//g' $(EXERCISE)/tests/Tests.elm
	$(GIT) add $(EXERCISE)/
	$(GIT) commit -m "Enable all tests"

test::
	$(ELM_TEST)

# coverage::

clean::
	$(RM) -rf elm-stuff
