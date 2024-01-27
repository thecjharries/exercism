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

# Create the feature branch
.PHONY: boot-feature-branch
boot-feature-branch: create
	$(GIT) checkout -b feat/$(TRACK)/$(EXERCISE)
	$(GIT) add $(EXERCISE)
	$(GIT) commit -m "Create $(TRACK):$(EXERCISE) exercise"

# Create exercise Makefile
.PHONY: boot-makefile
boot-makefile: boot-feature-branch
	echo "-include ../Makefile" >> $(EXERCISE)/Makefile
	echo "-include ../../.global.mk" >> $(EXERCISE)/Makefile
	$(GIT) add $(EXERCISE)/Makefile
	$(GIT) commit -m "Add Makefile"
