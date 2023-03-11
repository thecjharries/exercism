# Aliases for executables
CARGO ?= cargo
EXERCISM ?= exercism
FIND ?= find
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
SUBMISSIONS = src/lib.rs Cargo.toml
TRACK = rust

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
	$(EXERCISM) submit $(SUBMISSIONS)

# Finish the branch
.PHONY: push
push: submit
	$(GIT) push -u origin feat/$(TRACK)/$(EXERCISE)
