-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
# GIT ?= git
PIPENV ?= pipenv
# RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE_SNAKE_CASE).py
TRACK = python

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"
test::
	$(PIPENV) run pytest
# coverage::
clean::
	$(RM) -rf .pytest_cache
	$(RM) -rf $$($(PIPENV) --venv)



# Create the feature branch
.PHONY: boot
boot: create
	$(GIT) checkout -b feat/$(TRACK)/$(EXERCISE)
	$(GIT) add $(EXERCISE)
	$(GIT) commit -m "Create $(TRACK):$(EXERCISE) exercise"
	echo "-include ../Makefile" >> $(EXERCISE)/Makefile
	$(GIT) add $(EXERCISE)/Makefile
	$(GIT) commit -m "Add Makefile"
	$(CP) ./Pipfile $(EXERCISE)/Pipfile
	$(CD) $(EXERCISE) && $(PIPENV) install --dev
	$(GIT) add $(EXERCISE)/Pipfile $(EXERCISE)/Pipfile.lock
	$(GIT) commit -m "Add Pipfile"
	$(CODE) --reuse-window $(EXERCISE)/*.py
