use std::cmp;
use fastset::*;

// Temporary
macro_rules! info {
    ($( $x:expr ),*) => {
        
    };
}

#[no_mangle]
pub fn mu(n: u32, k: u32, l: u32) -> u32 {
    for m in 1..n {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            let mut k_a = a.hfoldsumset(k, n);
            let l_a = a.hfoldsumset(l, n);
            k_a.intersect(&l_a);
            if k_a.isempty() {
                info!("[mu] For m={}, found {}, which is sum-free", m, a);
                info!("[mu] (k_a = {}, l_a = {})", a.hfoldsumset(k, n), l_a);
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn mu_signed(n: u32, k: u32, l: u32) -> u32 {
    for m in 1..n {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            let mut k_a = a.hfoldsignedsumset(k, n);
            let l_a = a.hfoldsignedsumset(l, n);
            k_a.intersect(&l_a);
            if k_a.isempty() {
                info!("[mu] For m={}, found {}, which is sum-free", m, a);
                info!("[mu] (k_a = {}, l_a = {})", a.hfoldsignedsumset(k, n), l_a);
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn mu_restricted(n: u32, k: u32, l: u32) -> u32 {
    if k > n || l > n {
        return n;
    }
    let mut lower_bound = 1;
    if l == 1 && (n == k*(k*k - 1)) {
        lower_bound = cmp::max(n/(k + 1) + k - 1, k*k);
        info!("[mu] Using lower bound: {}", lower_bound);
    }
    for m in lower_bound..n {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            let mut k_a = a.hfoldrestrictedsumset(k, n);
            let l_a = a.hfoldrestrictedsumset(l, n);
            k_a.intersect(&l_a);
            if k_a.isempty() {
                info!("[mu] For m={}, found {}, which is sum-free", m, a);
                info!("[mu] (k_a = {}, l_a = {})", a.hfoldrestrictedsumset(k, n), l_a);
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn mu_signed_restricted(n: u32, k: u32, l: u32) -> u32 {
    if k > n || l > n {
        return n;
    }
    for m in 1..n {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            let mut k_a = a.hfoldrestrictedsignedsumset(k, n);
            let l_a = a.hfoldrestrictedsignedsumset(l, n);
            k_a.intersect(&l_a);
            if k_a.isempty() {
                info!("[mu] For m={}, found {}, which is sum-free", m, a);
                info!("[mu] (k_a = {}, l_a = {})", a.hfoldrestrictedsignedsumset(k, n), l_a);
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Based on page 358, 359
    #[test]
    fn test_mu_res() {
        assert_eq!(mu_restricted(9, 3, 1), 4);
        assert_eq!(mu_restricted(14, 3, 1), 5);
        assert_eq!(mu_restricted(19, 3, 1), 6);
        assert_eq!(mu_restricted(15, 4, 2), 5);
        assert_eq!(mu_restricted(12, 4, 3), 6);

        assert_eq!(mu_restricted(6, 8, 1), 6);
        assert_eq!(mu_restricted(11, 6, 1), 6);
    }
}