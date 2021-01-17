#![feature(test)]
extern crate test;

use plr::regression::GreedyPLR;
use plr::Segment;
use rand::Rng;
use std::cmp::min;
use std::cmp::Ordering;
use test::Bencher;

const N: usize = 100000;

#[bench]
fn bi_search(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let mut v = vec![];
    for i in 0..N {
        v.push((i as f64).powf(1.1) as usize);
    }
    // println!("{:?}", v);

    let index = v.binary_search_by(|probe| probe.cmp(&1297));
    println!("bi_search result -> {:?}", index);

    let mut correct_count = 0u32;
    let mut wrong_count = 0u32;
    let mut not_found_count = 0u32;
    let mut total_count = 0u32;
    b.iter(|| {
        let key = rng.gen_range(0..(((N - 1) as f64).powf(1.1) as usize));
        match v.binary_search_by(|probe| probe.cmp(&key)) {
            Ok(index) => {
                let expected = (index as f64).powf(1.1) as usize;
                if expected != key {
                    wrong_count += 1;
                } else {
                    correct_count += 1;
                }
            }
            Err(_) => {
                not_found_count += 1;
            }
        }
        total_count += 1;
    });
    println!(
        "Total count: {}, correct: {}, wrong: {}, not_found: {}, rate: {}",
        total_count,
        correct_count,
        wrong_count,
        not_found_count,
        correct_count as f64 / total_count as f64
    );
}

#[bench]
fn plr_search(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let mut v = vec![];
    for i in 0..N {
        v.push((i as f64).powf(1.1) as usize);
    }

    let segments = plr_train(&v, 8.0);
    println!("\n\nSegment size: {}", segments.len());

    let index = plr_predict(&segments, 1297, &v);
    println!("plr_search result {} -> {:?}", 1297, index);

    let mut correct_count = 0u32;
    let mut wrong_count = 0u32;
    let mut not_found_count = 0u32;
    let mut total_count = 0u32;
    b.iter(|| {
        let key = rng.gen_range(0..(((N - 1) as f64).powf(1.1) as usize));
        match plr_predict(&segments, key, &v) {
            Ok(index) => {
                let expected = (index as f64).powf(1.1) as usize;
                if expected != key {
                    wrong_count += 1;
                } else {
                    correct_count += 1;
                }
            }
            Err(_) => {
                not_found_count += 1;
            }
        }
        total_count += 1;
    });
    println!(
        "Total count: {}, correct: {}, wrong: {}, not_found: {}, rate: {}",
        total_count,
        correct_count,
        wrong_count,
        not_found_count,
        correct_count as f64 / total_count as f64
    );
}

fn plr_predict(segments: &Vec<Segment>, key: usize, v: &Vec<usize>) -> Result<usize, usize> {
    let keyf64 = key as f64;
    let segment_index = segments.binary_search_by(|probe| {
        if probe.stop <= keyf64 {
            return Ordering::Less;
        } else if probe.start > keyf64 {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    })?;

    let segment = segments.get(segment_index).unwrap();
    let index_pre = (segment.slope * keyf64 + segment.intercept) as usize;

    let left = index_pre.saturating_sub(8); // make left >= 0
    let right = min(v.len() - 1, index_pre + 8);
    v[left..right]
        .binary_search_by(|probe| probe.cmp(&key))
        .map(|index| index + left)
}

fn plr_train(v: &Vec<usize>, error_range: f64) -> Vec<Segment> {
    let mut plr = GreedyPLR::new(error_range); // gamma = 0.0005, the maximum regression error
    let mut segments = Vec::new();

    for (index, key) in v.iter().enumerate() {
        // when `process` returns a segment, we should add it to our list
        if let Some(segment) = plr.process(*key as f64, index as f64) {
            segments.push(segment);
        }
    }

    // because we have a finite amount of data, we flush the buffer and get the potential
    // last segment.
    if let Some(segment) = plr.finish() {
        segments.push(segment);
    }
    segments
}

// Output is like:
//
// cargo bench -- --nocapture
//    Compiling plr-test v0.1.0 (/Users/fuyangl/Workspace/tmp/plr-test)
//     Finished bench [optimized] target(s) in 0.42s
//      Running target/release/deps/plr_test-da582257737a6d90
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//
//      Running target/release/deps/bench1-239bdceb0d5136d9
//
// running 2 tests
// bi_search result -> Ok(676)
// Total count: 31518001, correct: 9971471, wrong: 0, not_found: 21546530, rate: 0.316373839825692
// test bi_search  ... bench:         115 ns/iter (+/- 9)
//
//
// Segment size: 26
// plr_search result 1297 -> Ok(676)
// Total count: 3129001, correct: 989874, wrong: 0, not_found: 2139127, rate: 0.3163546448211426
// test plr_search ... bench:          87 ns/iter (+/- 36)
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out
