-include ../global.mk

# Aliases for executables
DOTNET ?= dotnet
RM ?= rm

# Exercism variables
SUBMISSIONS = $(EXERCISE_PASCAL_CASE).fs
TRACK = fsharp

boot-tests::
	$(SED) -Ei 's/\(Skip[^)]+\)//g' $(EXERCISE)/$(EXERCISE_PASCAL_CASE)Tests.fs
	$(GIT) add $(EXERCISE)/$(EXERCISE_PASCAL_CASE)Tests.fs
	$(GIT) commit -m "Enable all tests"

test::
	$(DOTNET) test

clean::
	$(RM) -rf bin obj
