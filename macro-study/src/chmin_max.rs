#[macro_export]
macro_rules! chmin {
    ($min: expr, $v: expr) => {
        if $v < $min {
            $min = $v;
            true
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! chmax {
    ($max: expr, $v: expr) => {
        if $v > $max {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn expand() {
    let mut a: i32 = 10;
    chmin!(a, 5);
    chmax!(a, 6);
}
