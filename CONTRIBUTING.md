# Contributing to rust-comp

## Setting up

1. Clone the repository.
2. Install cargo.
3. Install cargo-fmt.
4. Copy the files in `git/hooks` to `.git/hooks`.  This will ensure you commit
   and push working code.

To verify you have it installed correctly, run `./verify`.  This script also
verifies that the code is formatted correctly and all the tests pass.

## Developing

There are many areas to help with development.  When you commit, the hooks you
set up previously will ensure all tests are passing and the code is formatted
correctly.

### Workflow

This project is being developed using Test Driven Development (TDD).  That means
that unless your feature is intrinsically linked to IO, you should be writing
tests at the same time as you write the actual code.  This will ensure your code
continues to work over time even as the rest of the system evolves.

    RUST_BACKTRACE=1 cargo test --all

### Writing tests

To ensure that your tests are still useful as time do the following things:
+ Make each test have exactly 1 purpose
+ Split functions into small pieces so tests can achieve the above goal
+ Refactor common behavior out of tests.
+ Give your test functions long names that describe what the take as inputs and
  what they expect as output in english.

And avoid all of the following things:
- **Do not copy and paste code used commonly in your tests.**
  * Solution: Write a function that does the common things.  If it isn't
    possible to do that because of lifetime problems, you've come across a
    design flaw of the current system and it should be fixed.
- Writing tests that do too many things.
  * Solution: Write multiple tests.
  * Pros: Easier to understand what the bug is when a test fails.

To ensure your code is easily testable do the following things:
* Split your code into small functions.
* Separate IO interaction from main line code.

Tests are placed at the bottom of files in the `tests` module:

    pub fn identity(x: i32) -> i32 { x }
    
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_identity() {
            assert_eq!(identity(1), 1);
        }
    }

Almost all tests should be unit tests.  That is, they test exactly one function
and do not interact with IO at all.  This has the following advantages:
+ Tests are faster to run because they do less.
+ When tests fail, it is clearer what is going wrong.

### Code style and formatting

Code should be formatted using the tool cargo-fmt.  Run it as `cargo fmt --all`
flag to auto format all files.

Do not write documentation comments that say what you are doing!  I can read the
code.  If your task is unintuitive or your function is growing large, split it
into multiple functions that each have a descriptive name.  This is much more
useful because it does the documentation for you and reduces the visual noise.
This also makes the tests simpler as they should be testing those subroutines
rather than the overall function.

Don't test data.  We know data is valid because it's data.  Instead, test the
behavior of the code.

Assertions should assert in the following order (this is the opposite of other
programming languages):

    assert_eq!(actual_value, expected_value);

Tests should start with `test_`.  If there is only one possible test for a
function, name your test `test_name_of_function`.  If there are multiple, name
them like `test_name_of_function_empty_file_errors`.  That is `test_` + nameof
function + given condition + expected output.

### Optimizing

One of the stated goals of the project is to create a fast rust compiler.  Thus
I recommend trying to optimize the compiler and comparing the compiler to the
builtin implementation.

Techniques for performance testing this code base and optimizing the code are
located in [optimizing.md](optimizing.md).
