-include ../global.mk

# Aliases for executables
DOTNET ?= dotnet
RM ?= rm

# Exercism variables
SUBMISSIONS = $(EXERCISE_PASCAL_CASE).fs
TRACK = fsharp

test::
	$(DOTNET) test

clean::
	$(RM) -rf bin obj
