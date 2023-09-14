# Aliases for executables
BATS ?= bats
CODE ?= code
EXERCISM ?= exercism
FIND ?= find
GH ?= gh
GIT ?= git
RM ?= rm
SED ?= sed
XDG_OPEN ?= xdg-open

# Exercism variables
ifeq (,$(IN))
EXERCISE = $(shell basename $(CURDIR))
else
EXERCISE = $(shell echo $(IN) | sed 's/.*--exercise=\([^ ]*\) --.*/\1/')
endif
TRACK = jq

# Debug the variables
.PHONY: debug
debug:
	@echo "EXERCISE: $(EXERCISE)"
	@echo "IN: $(IN)"
	@echo "SUBMISSION: $(SUBMISSION)"
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
	$(CODE) --reuse-window $(EXERCISE)/*.jq
	$(FIND) ./$(EXERCISE)/test*.bats -type f -name 'test*.bats' -exec $(SED) -i '/BATS_RUN_SKIPPED/d' {} \;
	$(GIT) add $(EXERCISE)/test*.bats
	$(GIT) commit -m "Enable all tests" || exit 0

# Run the tests
.PHONY: test
test:
	$(BATS) ./test*.bats

# Remove any built artifacts
.PHONY: clean
clean:
	echo "Nothing to clean"

# Submit the solution
.PHONY: submit
submit:
	$(GIT) diff-index --quiet HEAD || (echo "Uncommitted changes"; exit 1)
	$(EXERCISM) submit *.jq

# Finish the branch
.PHONY: finish
finish: test submit clean
	$(GIT) push -u origin feat/$(TRACK)/$(EXERCISE)
	$(GH) pr create --fill
	$(GH) pr merge --merge --delete-branch
