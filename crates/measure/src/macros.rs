#[macro_export]
macro_rules! measure_time {
    ($val:expr, $name:tt $(,)?) => {{
        let mut measure = $crate::measure::Measure::start($name);
        let result = $val;
        measure.stop();
        (result, measure)
    }};
    ($val:expr) => {
        measure_time!($val, "")
    };
}

#[macro_export]
macro_rules! measure_us {
    ($expr:expr) => {{
        let (result, duration) = $crate::meas_dur!($expr);
        (result, 0)
    }};
}

#[macro_export]
macro_rules! meas_dur {
    ($expr:expr) => {{
        let result = $expr;
        // (result, start.elapsed())
        (result, 0)
    }};
}
