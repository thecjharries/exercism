# exercism

This repo contains solutions to [exercism](https://exercism.org) exercises.

## Online Tracks

I (so far) do not do these tracks locally for a variety of reasons.

- [Gleam](https://exercism.org/tracks/gleam): The Gleam compiler won't build in Rust and provides zero documentation for fixes
- [MIPS Assembly](https://exercism.org/tracks/mips): I don't want to deal with assembly
- [Pharo](https://exercism.org/tracks/pharo-smalltalk): I'm not dealing with their convoluted VM setup

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
