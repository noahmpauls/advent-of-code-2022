# Advent of Code 2022

My solutions for Advent of Code 2022. This year I am using AoC to learn the Vim editor, along with a review of Rust (I'm pretty rusty on it).

## Starting a new day

Each day is contained in a package within the overall workspace. To automatically create a new day, run `./scripts/new-day.ps1` from the workspace root:

```
> ./scripts/new-day.ps1 --day 01 --name name_of_day
```

This scripts performs the following actions:
1. Runs a utility to add the new project to the list of workspace members.
2. Creates the new project folder and copies the template files into it.
3. Updates the new project's name in its `Cargo.toml` file.

## Running a given day's solutions

Run a day's solutions using the script file `./scripts/run-day.ps1` from the workspace root:

```
> .\scripts\run-day.ps1 01 2 input
```

The above command runs day 1 part 2, with the `input.txt` file in the package's `input` folder as the program input. You can add the `-release` flag to run the release version of the package, which may run faster. The `-release` flag will always trigger a rebuild, which can be skipped using the `-skipbuild` flag.

