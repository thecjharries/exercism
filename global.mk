# Debug the variables
.PHONY: debug
debug::
	@echo "EXERCISE: $(EXERCISE)"
	@echo "IN: $(IN)"
	@echo "SUBMISSIONS: $(SUBMISSIONS)"
	@echo "TRACK: $(TRACK)"

# Create the exercise
.PHONY: create
create:
	$(EXERCISM) download --exercise=$(EXERCISE) --track=$(TRACK)
