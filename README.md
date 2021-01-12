# plr-test

Testing using crate https://crates.io/crates/plr to speed up binary search of a sorted vec.

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
