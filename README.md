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
