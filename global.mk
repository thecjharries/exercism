# Aliases for executables
BASENAME ?= basename
CODE ?= code
CP ?= cp
EXERCISM ?= exercism
GH ?= gh
GIT ?= git
MKDIR ?= mkdir
SED ?= sed

# Exercism variables
ifeq (,$(IN))
EXERCISE = $(shell $(BASENAME) $(CURDIR))
else
EXERCISE = $(shell echo $(IN) | $(SED) 's/.*--exercise=\([^ ]*\)/\1/')
endif
EXERCISE_SNAKE_CASE = $(shell echo $(EXERCISE) | $(SED) 's/-/_/g')
EXERCISE_PASCAL_CASE = $(shell echo $(EXERCISE) | $(SED) 's/\(^\|-\)\([a-z]\)/\U\2/g')
SUBMISSIONS =
TRACK =

# Debug the variables
.PHONY: debug
debug::
	@echo "EXERCISE: $(EXERCISE)"
	@echo "EXERCISE_SNAKE_CASE: $(EXERCISE_SNAKE_CASE)"
	@echo "EXERCISE_PASCAL_CASE: $(EXERCISE_PASCAL_CASE)"
	@echo "IN: $(IN)"
	@echo "SUBMISSIONS: $(SUBMISSIONS)"
	@echo "TRACK: $(TRACK)"

# Create the exercise
.PHONY: create
create: guard-EXERCISE guard-TRACK
	@echo "Creating exercise..."
	$(EXERCISM) download --exercise=$(EXERCISE) --track=$(TRACK)

# Boot track
.PHONY: boot-track
boot-track: guard-TRACK
	@echo "Booting track..."
	$(GIT) checkout -b feat/setup-$(TRACK)
	$(MKDIR) -p $(TRACK)
	$(CP) ./child.mk $(TRACK)/Makefile
	$(SED) -i 's/@@TRACK@@/$(TRACK)/g' $(TRACK)/Makefile
	$(GIT) add $(TRACK)/Makefile
	$(GIT) commit -m "Add $(TRACK) Makefile"


# Create the feature branch
.PHONY: boot-feature-branch
boot-feature-branch: guard-TRACK guard-EXERCISE create
	@echo "Creating feature branch..."
	$(GIT) checkout -b feat/$(TRACK)/$(EXERCISE)
	$(GIT) add $(EXERCISE)
	$(GIT) commit -m "Create $(TRACK):$(EXERCISE) exercise"

# Create exercise Makefile
.PHONY: create-makefile
create-makefile: guard-EXERCISE
	@echo "Creating Makefile..."
	echo "-include ../../global.mk" >> $(EXERCISE)/Makefile
	echo "-include ../Makefile" >> $(EXERCISE)/Makefile

# Create exercise Makefile
.PHONY: boot-makefile
boot-makefile: guard-EXERCISE boot-feature-branch create-makefile
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
	for submission in $(SUBMISSIONS); do \
		$(CODE) --reuse-window $(EXERCISE)/$$submission & \
	done

# Full boot target
.PHONY: boot
boot:: create boot-feature-branch boot-makefile boot-tests boot-vscode
	@echo "Boot complete."

# Convenience target for bats tests
.PHONY: test-bats
test-bats:
	BATS_RUN_SKIPPED=true $(BATS) *.bats

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

# Ensure no uncommitted changes
.PHONY: ensure-committed
ensure-committed::
	$(GIT) diff-index --quiet HEAD || (echo "Uncommitted changes"; exit 1)

# Submit the solution
.PHONY: submit
submit:: guard-SUBMISSIONS ensure-committed
	@echo "Submitting solution..."
	$(EXERCISM) submit $(SUBMISSIONS)

# Finish the branch
.PHONY: finish
finish:: guard-TRACK guard-EXERCISE coverage submit clean
	@echo "Finishing up..."
	$(GIT) push -u origin feat/$(TRACK)/$(EXERCISE)
	$(GH) pr create --fill
	$(GH) pr merge --merge --delete-branch

# Unconnected finish
.PHONY: finish-unconnected
finish-unconnected: ensure-committed
	@echo "Finishing up..."
	$(GIT) push -u origin HEAD
	$(GH) pr create --fill
	$(GH) pr merge --merge --delete-branch

# Guard against missing environment variables
# https://stackoverflow.com/a/7367903
.PHONY: guard-%
guard-%:
	@ if [ "${${*}}" = "" ]; then \
		echo "Environment variable $* not set"; \
		exit 1; \
	fi
