# exercism

This repo contains solutions to [exercism](https://exercism.org) exercises.

## Future Work

These notes talk about how to test and submit. I will eventually (maybe) write Makefiles to automate these, especially if the language appears more than once.

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
