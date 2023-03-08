# Aliases for executables
CARGO ?= cargo
GIT ?= git
XDG_OPEN ?= xdg-open
RM ?= rm
EXERCISM ?= exercism
FIND ?= find
SED ?= sed

# Exercism variables
SUBMISSION = src/lib.rs
TRACK = rust
ifeq (,$(IN))
EXERCISE = $(shell basename $(CURDIR))
else
EXERCISE = $(shell echo $(IN) | sed 's/.*--exercise=\([^ ]*\) --.*/\1/')
endif

# Debug the variables
.PHONY: debug
debug:
	@echo "EXERCISE: $(EXERCISE)"
	@echo "SUBMISSION: $(SUBMISSION)"
	@echo "TRACK: $(TRACK)"
	@echo "IN: $(IN)"


# Create the exercise
.PHONY: create
create:
	$(EXERCISM) download --exercise=$(EXERCISE) --track=$(TRACK)

# Create the feature branch
.PHONY: branch
boot: create
	$(GIT) checkout -b feat/$(TRACK)/$(EXERCISE)
	$(GIT) add $(EXERCISE)
	$(GIT) commit -m "Create $(TRACK):$(EXERCISE) exercise"
	echo "-include ../Makefile" >> $(EXERCISE)/Makefile
	$(GIT) add $(EXERCISE)/Makefile
	$(GIT) commit -m "Add Makefile"
	$(FIND) ./$(EXERCISE)/tests/ -type f -name '*.rs' -exec $(SED) -i '/#\[ignore]/d' {} \;
	$(GIT) add $(EXERCISE)/tests/*.rs
	$(GIT) commit -m "Enable all tests"

# Run the tests
.PHONY: test
test:
	$(CARGO) test

# Get code coverage
.PHONY: coverage
coverage:
	$(CARGO) tarpaulin -v --fail-under=100

# Build coverage report
.PHONY: coverage-report
coverage-report:
	$(CARGO) tarpaulin -v --fail-under=100 --out HTML; $(XDG_OPEN) tarpaulin-report.html

# Remove any built artifacts
.PHONY: clean
clean:
	$(RM) -rf target
	$(RM) -rf Cargo.lock
	$(RM) -rf tarpaulin-report.html
	$(RM) -rf main


# Submit the solution
.PHONY: submit
submit: coverage clean
	$(EXERCISM) submit $(SUBMISSION)

# Finish the branch
.PHONY: push
push: submit
	$(GIT) push -u origin feat/$(TRACK)/$(EXERCISE)
