# plr-test

Testing using crate https://crates.io/crates/plr to speed up binary search of a sorted vec.

## How to build and use the binary tool
With input file format like
```
1,1
2,40
3,60
4,80
5,502
6,612
7,786
8,824
```

Build the cmd tool:
```
cargo build --release
./target/release/plr --help
plr 0.1.0
A Greedy Piecewise Logistic Regression Tool

USAGE:
    plr [OPTIONS] --input-file <input-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --error-bound <error-bound>     [default: 8.0]
    -i, --input-file <input-file>
```

Use the cmd tool:
```
./target/release/plr -i ./input1.txt
[{"start":1.0,"stop":5.0,"slope":25.0,"intercept":-17.0},{"start":5.0,"stop":7.0,"slope":110.0,"intercept":-48.0},{"start":7.0,"stop":1.7976931348623157e308,"slope":38.0,"intercept":520.0}]
```



## Bench output
```
cargo bench -- --nocapture
   Compiling plr-test v0.1.0 (/Users/fuyangl/Workspace/tmp/plr-test)
    Finished bench [optimized] target(s) in 0.65s
     Running target/release/deps/plr_test-da582257737a6d90

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/bench1-239bdceb0d5136d9

running 2 tests
bi_search result -> Ok(676)
Total count: 1759501, correct: 555507, wrong: 0, not_found: 1203994, rate: 0.31571849064024404
test bi_search  ... bench:         120 ns/iter (+/- 32)


Segment size: 26
plr_search result 1297 -> Ok(676)
Total count: 55414801, correct: 17521386, wrong: 0, not_found: 37893415, rate: 0.3161860312373945
test plr_search ... bench:          86 ns/iter (+/- 9)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out
```
