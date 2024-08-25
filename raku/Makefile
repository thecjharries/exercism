-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
# GIT ?= git
RAKU ?= raku
# RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = lib/$(EXERCISE_PASCAL_CASE).rakumod
TRACK = raku

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"
test::
	$(RAKU) t/$(EXERCISE).rakutest
# coverage::
# clean::
