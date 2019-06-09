# Performance testing rust code
## Setup

* Install `perf`
* Install `flamegraph` (optional)

## Build

Build in release mode with debug symbols by putting in `Cargo.toml`:

    [profile.release]
    debug = true

Force a full stack trace to be available through the frame pointer (required for
`perf` to correctly harvest where your code is running):

    RUSTFLAGS="-C force-frame-pointers" cargo build --release

## Make a source file to test against

If you already have a file, skip this stop and go to [Run](#Run)

If you don't have a file set up to performance test your code with, follow these
steps:
1. Make the report crate
   * Go to the root of the rust-comp project
   * Run `cargo new --lib report`
2. Open in a text editor create a new file named `report/src/backup.rs`
   * Put inside it around 1/2 million lines of code.
   * This code should be the same snippet repeated over and over.  For example:

```
pub fn f(x: i32) {
    let y = x;
    while y + x * y != 1 {
        let z = x;
        if { z == y } { ; }
    }

    let y = x;
    if x == y {
        let a = x;
        while y == a {
            let _z = x * (a + 2);
        }
    }
}
```

   * Make top level declarations public to avoid warnings / optimizations.
   * Number each code block by running the following.  Replace the query (`/pub
     fn f/`) and the replaced line (`print "pub fn f" ...`) to customize to your
     needs.

```
cat backup.rs | awk '/pub fn f/{ x++; print "pub fn f" x "(x: i32) {" } /^(}| |$)/{ print $0 }' > lib.rs
```

You now have set up `lib.rs` to be a syntactically valid piece of code!  Now you
can run the compiler against it.

## Run

Analyze the program by directly running it:

    ./target/release/rust-comp <ARGUMENTS>

Record a run with perf:

    perf record -g ./target/release/rust-comp <ARGUMENTS>

## Analyze

Flamegraph is a great tool that allows you to visualize the code flow and see
which functions are taking up the most time.  Work on optimizing the functions
that take up time not the ones that don't.  Display the results with flamegraph
(then open `flame.svg` in your browser):

    perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg

Display the results with perf:

    perf report -g "graph,0.5,caller"

More `perf report` details in this talk by Chandler Carruth
https://www.youtube.com/watch?v=nXaxk27zwlk.

## Cleanup

Delete the lines added to `Cargo.toml`.
