-include ../global.mk

# Aliases for executables
CD ?= cd
CP ?= cp
# GH ?= gh
GIT ?= git
PIPENV ?= pipenv
# RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE_SNAKE_CASE).py
TRACK = python

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"
boot-final::
	$(CP) ./Pipfile $(EXERCISE)/Pipfile
	$(CD) $(EXERCISE) && $(PIPENV) install --dev
	$(GIT) add $(EXERCISE)/Pipfile $(EXERCISE)/Pipfile.lock
	$(GIT) commit -m "Add Pipfile"
test::
	$(PIPENV) run pytest
# coverage::
clean::
	$(RM) -rf .pytest_cache
	$(RM) -rf $$($(PIPENV) --venv)
