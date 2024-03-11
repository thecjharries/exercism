-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
# GIT ?= git
RM ?= rm
# SED ?= sed
STACK ?= stack

HASKELL_FILENAME = $(shell L1=$(EXERCISE); echo $${L1^})

# Exercism variables
SUBMISSIONS = src/$(HASKELL_FILENAME).hs package.yaml
TRACK = haskell

boot-tests::
	cd $(EXERCISE) && $(STACK) build --test --no-run-tests
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"

test::
	$(STACK) test

# coverage::

clean::
	$(STACK) clean
	$(RM) -rf .stack-work $(EXERCISE).cabal stack.yaml.lock
