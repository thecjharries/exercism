-include ../global.mk

# Aliases for executables
# CP ?= cp
DOTNET ?= dotnet
# GH ?= gh
# GIT ?= git
RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE_PASCAL_CASE).cs
TRACK = csharp

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"

test::
	$(DOTNET) test

# coverage::

clean::
	$(RM) -rf bin/ obj/
