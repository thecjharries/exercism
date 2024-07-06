-include ../global.mk

# Aliases for executables
GIT ?= git
SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE_PASCAL_CASE).vb
TRACK = vbnet

boot-tests::
	$(SED) -Ei 's/\(Skip[^)]+\)//g' $(EXERCISE)/$(EXERCISE_PASCAL_CASE)Tests.vb
	$(GIT) add $(EXERCISE)/$(EXERCISE_PASCAL_CASE)Tests.vb
	$(GIT) commit -m "Enable all tests" || exit 0

test:: test-dotnet

# coverage::

clean:: clean-dotnet
