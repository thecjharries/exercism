-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
# GIT ?= git
RM ?= rm
# SED ?= sed
SWIFT ?= swift

# Exercism variables
SUBMISSIONS = Sources/$(EXERCISE_PASCAL_CASE)/$(EXERCISE_PASCAL_CASE).swift
TRACK = swift

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"

test::
	RUNALL=true $(SWIFT) test

# coverage::

clean::
	$(RM) -rf .build
