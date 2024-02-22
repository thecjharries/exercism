-include ../global.mk

# Aliases for executables
CODE ?= code
# CP ?= cp
# GH ?= gh
# GIT ?= git
RM ?= rm
# SED ?= sed
SWIFT ?= swift

# Exercism variables
SUBMISSIONS = Sources/$(EXERCISE_PASCAL_CASE).swift
TRACK = swift

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"

boot-vscode::
	$(CODE) --reuse-window $(SUBMISSIONS)

test::
	$(SWIFT) test

# coverage::

clean::
	$(RM) -rf .build
