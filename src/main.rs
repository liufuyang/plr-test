use plr::GreedyPLR;

fn main() {

    // first, generate some data points...
    let mut data = Vec::new();

    for i in 0..1000 {
        let x = (i as f64) / 1000.0 * 7.0;
        let y = f64::sin(x);
        data.push((i, y));
    }

    let mut plr = GreedyPLR::new(0.05); // gamma = 0.0005, the maximum regression error

    let mut segments = Vec::new();

    for (x, y) in data {
        // when `process` returns a segment, we should add it to our list
        if let Some(segment) = plr.process(x as f64, y) {
            segments.push(segment);
        }
    }

    // because we have a finite amount of data, we flush the buffer and get the potential
    // last segment.
    if let Some(segment) = plr.finish() {
        segments.push(segment);
    }

    println!("{:?}", segments);
    // Output something like [Segment { start: 0.0, stop: 135.0, slope: 0.006199380974600129, intercept: 0.00040028092943662774 }, Segment { start: 135.0, stop: 229.0, slope: 0.002289919285569413, intercept: 0.5022530253803776 }, Segment { start: 229.0, stop: 324.0, slope: -0.002177701114062232, intercept: 1.4991389761520562 }, Segment { start: 324.0, stop: 485.0, slope: -0.006165159252149305, intercept: 2.7649781746856226 }, Segment { start: 485.0, stop: 602.0, slope: -0.005591045295367808, intercept: 2.460363429406417 }, Segment { start: 602.0, stop: 694.0, slope: -0.0014847707591850395, intercept: 0.014558330290503019 }, Segment { start: 694.0, stop: 792.0, slope: 0.002954175556240355, intercept: -3.04057240156199 }, Segment { start: 792.0, stop: 1000.0, slope: 0.006441599299829636, intercept: -5.776058714730171 }]
}
