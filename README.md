# exercism

This repo contains solutions to [exercism](https://exercism.org) exercises.

## Future Work

These notes talk about how to test and submit. I will eventually (maybe) write Makefiles to automate these, especially if the language appears more than once.

### Clojure

```bash
clj -X:test
```

Submit `src/<snake_case_exercise>.clj`

### C++

Set up the tests:

```bash
echo 'build/' >> .gitignore
mkdir build
cd build
cmake -G "Unix Makefiles" ..
```

Run the tests:

```bash
cd build
make
```

Submit `<snake_case_exercise>.{h,cpp}`

### Nim

```bash
nim r test_<snake_case_exercise>.nim
```

Submit `<snake_case_exercise>.nim`

### R

- Ensure `.Renviron` is set up
- Make sure `testthat` is installed

  ```bash
  Rscript -e 'install.packages("testthat")'
  ```

Submit `<dash-case-exercise>.R`

### Ruby

- Ensure `gem install minitest` has been run
- `sed -i -e '/skip$/d' <snake_case_exercise>_test.rb`
- `ruby -r minitest/pride <snake_case_exercise>_test.rb`

Submit `<snake_case_exercise>.rb`
