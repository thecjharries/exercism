# Aliases for executables
BASENAME ?= basename
CODE ?= code
EXERCISM ?= exercism
GH ?= gh
GIT ?= git
SED ?= sed

# Exercism variables
ifeq (,$(IN))
EXERCISE = $(shell $(BASENAME) $(CURDIR))
else
EXERCISE = $(shell echo $(IN) | $(SED) 's/.*--exercise=\([^ ]*\)/\1/')
endif
EXERCISE_SNAKE_CASE = $(shell echo $(EXERCISE) | $(SED) 's/-/_/g')
SUBMISSIONS =
TRACK =

# Debug the variables
.PHONY: debug
debug::
	@echo "EXERCISE: $(EXERCISE)"
	@echo "EXERCISE_SNAKE_CASE: $(EXERCISE_SNAKE_CASE)"
	@echo "IN: $(IN)"
	@echo "SUBMISSIONS: $(SUBMISSIONS)"
	@echo "TRACK: $(TRACK)"

# Create the exercise
.PHONY: create
create:
	@echo "Creating exercise..."
	$(EXERCISM) download --exercise=$(EXERCISE) --track=$(TRACK)

# Create the feature branch
.PHONY: boot-feature-branch
boot-feature-branch: create
	@echo "Creating feature branch..."
	$(GIT) checkout -b feat/$(TRACK)/$(EXERCISE)
	$(GIT) add $(EXERCISE)
	$(GIT) commit -m "Create $(TRACK):$(EXERCISE) exercise"

# Create exercise Makefile
.PHONY: create-makefile
create-makefile:
	@echo "Creating Makefile..."
	echo "-include ../../global.mk" > $(EXERCISE)/Makefile
	echo "-include ../Makefile" >> $(EXERCISE)/Makefile

# Create exercise Makefile
.PHONY: boot-makefile
boot-makefile: boot-feature-branch create-makefile
	@echo "Adding Makefile..."
	$(GIT) add $(EXERCISE)/Makefile
	$(GIT) commit -m "Add Makefile"

# Create exercise tests
.PHONY: boot-tests
boot-tests::
	@echo "Creating tests..."

# Launch VS Code
.PHONY: boot-vscode
boot-vscode::
	@echo "Launching VS Code..."

# Full boot target
.PHONY: boot
boot:: create boot-feature-branch boot-makefile boot-tests boot-vscode
	@echo "Boot complete."

# Run the tests
.PHONY: test
test::
	@echo "Running tests..."

# Get code coverage
.PHONY: coverage
coverage::
	@echo "Getting code coverage..."

# Remove any built artifacts
.PHONY: clean
clean::
	@echo "Cleaning up..."

# Submit the solution
.PHONY: submit
submit::
	@echo "Submitting solution..."
	$(GIT) diff-index --quiet HEAD || (echo "Uncommitted changes"; exit 1)
	$(EXERCISM) submit $(SUBMISSIONS)

# Finish the branch
.PHONY: finish
finish:: coverage submit clean
	@echo "Finishing up..."
	$(GIT) push -u origin feat/$(TRACK)/$(EXERCISE)
	$(GH) pr create --fill
	$(GH) pr merge --merge --delete-branch
