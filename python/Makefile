# Aliases for executables
CD ?= cd
CODE ?= code
CP ?= cp
EXERCISM ?= exercism
GH ?= gh
GIT ?= git
GREP ?= grep
LS ?= ls
PIPENV ?= pipenv
RM ?= rm
SED ?= sed
XDG_OPEN ?= xdg-open

# Exercism variables
ifeq (,$(IN))
EXERCISE = $(shell basename $(CURDIR))
else
EXERCISE = $(shell echo $(IN) | sed 's/.*--exercise=\([^ ]*\)/\1/')
endif
# find's -maxdepth flag is not universal
SUBMISSIONS = $(shell $(LS) | $(GREP) '.py' | $(GREP) -v 'test.py')
TRACK = python

# Debug the variables
.PHONY: debug
debug:
	@echo "EXERCISE: $(EXERCISE)"
	@echo "IN: $(IN)"
	@echo "SUBMISSIONS: $(SUBMISSIONS)"
	@echo "TRACK: $(TRACK)"

# Create the exercise
.PHONY: create
create:
	$(EXERCISM) download --exercise=$(EXERCISE) --track=$(TRACK)

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
	$(CODE) --reuse-window $(EXERCISE)/src/lib.rs $(EXERCISE)/tests/*.rs

# Run the tests
.PHONY: test
test:
	$(PIPENV) run pytest

# Get code coverage
.PHONY: coverage
coverage:
	@echo "Coverage is not supported for Python yet"

# Remove any built artifacts
.PHONY: clean
clean:
	$(RM) -rf .pytest_cache
	$(RM) -rf $$($(PIPENV) --venv)

# Submit the solution
.PHONY: submit
submit:
	$(GIT) diff-index --quiet HEAD || (echo "Uncommitted changes"; exit 1)
	$(EXERCISM) submit $(SUBMISSIONS)

# Finish the branch
.PHONY: finish
finish: coverage submit clean
	$(GIT) push -u origin feat/$(TRACK)/$(EXERCISE)
	$(GH) pr create --fill
	$(GH) pr merge --merge --delete-branch
