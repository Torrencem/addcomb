use std::cmp;
use fastset::*;
use comb::*;

// Temporary
macro_rules! info {
    ($( $x:expr ),*) => {
        
    };
}

#[no_mangle]
pub fn phi(n: u32, h: u32) -> u32 {
    phi_interval(n, (0, h)) + 1
}

#[no_mangle]
pub fn phi_interval(n: u32, intv: (u32, u32)) -> u32 {
    let mut lower_bound = 1;
    // Proposition B.10
    if intv.0 == 0 {
        let s = intv.1;
        lower_bound = cmp::max(1, (((factorial(s) * n) as f32).powf(1f32/(s as f32)).ceil() as i32) - (s as i32)) as u32;
        info!("[phi] (Proposition B.10) Using lower bound: {}", lower_bound);
    }

    for m in lower_bound.. {
        for a in each_set_exact(n, m) {
            if a.hfoldintervalsumset(intv, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

// TODO: Maybe impliment f and g functions on page 132
//(need an upper bound on n though, maybe read paper?)

#[no_mangle]
pub fn phi_signed(n: u32, h: u32) -> u32 {
    for m in 2u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldsignedsumset(h, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn phi_signed_interval(n: u32, intv: (u32, u32)) -> u32 {
    for m in 1u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldintervalsignedsumset(intv, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

// Not a very researched function... (page 145)
#[no_mangle]
pub fn phi_restricted(n: u32, h: u32) -> u32 {
    for m in 2u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldrestrictedsumset(h, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn phi_restricted_interval(n: u32, intv: (u32, u32)) -> u32 {
    let mut lower_bound = 1u32;
    // Proposition B.73
    if intv == (0, 2) {
        lower_bound = ((((8*n - 7) as f32).sqrt() - 1.0)/2.0).ceil() as u32;
        info!("[phi] (Proposition B.73) Using lower bound: {}", lower_bound);
    }
    for m in lower_bound.. {
        for a in each_set_exact(n, m) {
            if a.hfoldintervalrestrictedsumset(intv, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn phi_signed_restricted(n: u32, h: u32) -> u32 {
    for m in 2u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldrestrictedsignedsumset(h, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn phi_signed_restricted_interval(n: u32, intv: (u32, u32)) -> u32 {
    for m in 1u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldintervalrestrictedsignedsumset(intv, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    // From table on page 129
    #[test]
    pub fn test_phi_interval() {
        assert_eq!(phi_interval(17, (0, 2)), 5);
        assert_eq!(phi_interval(18, (0, 2)), 6);
        assert_eq!(phi_interval(9, (0, 3)),  3);
        assert_eq!(phi_interval(8, (0, 3)),  2);
        assert_eq!(phi_interval(26, (0, 4)), 3);
        assert_eq!(phi_interval(49, (0, 4)), 4);
    }

    // From table on page 147
    #[test]
    pub fn test_phi_restricted_interval() {
        assert_eq!(phi_restricted_interval(10, (0, 2)), 4);
        assert_eq!(phi_restricted_interval(3, (0, 2)), 2);
        assert_eq!(phi_restricted_interval(5, (0, 3)), 3);
        assert_eq!(phi_restricted_interval(8, (0, 4)), 3);
        assert_eq!(phi_restricted_interval(9, (0, 4)), 4);
    }
}