# SAT portfolio

## Description
This Rust library is a collection of SAT solvers, along with an interface to create your own and all the needed data structures.

There are two types of solvers:
- **Internal SAT solvers** made specifically for this library.
- **External SAT solvers** that can be used with this library.

## Available solvers
### External
- Minisat
- Manysat
- Glucose
### Internal
- DPLL

## Authors
- Christophe Yang
- Quentin Januel
- Sylvain Declercq

## Prerequisites
### MacOS
You need to have the `libomp` library installed. If you don't, you can install it with
```bash
brew install libomp
```

## Git submodules
Make sure to clone the submodules. If you haven't, just run
```bash
git submodule update --init --recursive
```
