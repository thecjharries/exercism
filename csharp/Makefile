-include ../global.mk

# Aliases for executables
GIT ?= git
SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE_PASCAL_CASE).cs
TRACK = csharp

boot-tests::
	$(SED) -Ei 's/\(Skip[^)]+\)//g' $(EXERCISE)/$(EXERCISE_PASCAL_CASE)Tests.cs
	$(GIT) add $(EXERCISE)/$(EXERCISE_PASCAL_CASE)Tests.cs
	$(GIT) commit -m "Enable all tests"

test:: test-dotnet

# coverage::

clean:: clean-dotnet
