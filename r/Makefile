-include ../global.mk

# Aliases for executables
RSCRIPT ?= Rscript

# Exercism variables
SUBMISSIONS = $(EXERCISE).R
TRACK = r

# Ensure we have the proper test dependency
boot-tests::
	Rscript -e 'if (!require("testthat")) install.packages("testthat")'

test::
	$(RSCRIPT) test*.R
