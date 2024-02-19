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

boot-tests::
	$(SED) -Ei 's/\(Skip[^)]+\)//g' $(EXERCISE)/$(EXERCISE_PASCAL_CASE)Tests.cs
	$(GIT) add $(EXERCISE)/$(EXERCISE_PASCAL_CASE)Tests.cs
	$(GIT) commit -m "Enable all tests"

test::
	$(DOTNET) test

# coverage::

clean::
	$(RM) -rf bin/ obj/
