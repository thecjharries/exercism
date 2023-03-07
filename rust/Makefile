# Aliases for executables
CARGO ?= cargo
GIT ?= git
XDG_OPEN ?= xdg-open
RM ?= rm
EXERCISM ?= exercism

# Exercism variables
SUBMISSION = src/lib.rs
TRACK = rust
EXERCISE = $(shell basename $(CURDIR))

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
submit:
	$(EXERCISM) submit $(SUBMISSION)

# Finish the branch
.PHONY: push
push: coverage clean submit
	$(GIT) push -u origin feat/$(TRACK)/$(EXERCISE)
