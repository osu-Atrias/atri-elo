/// Solve f(x) = 0 where x belongs to [a, b].
///
/// Panics when `a < b` or `f(a) < 0 < f(b)` is not satisfied.
pub fn solve_itp(mut f: impl FnMut(f64) -> f64, (mut a, mut b): (f64, f64), epsilon: f64) -> f64 {
    const N_0: usize = 1;

    debug_assert!(a < b);

    let mut y_a = f(a);
    let mut y_b = f(b);

    if y_a.abs() < epsilon {
        return a;
    } else if y_b.abs() < epsilon {
        return b;
    }

    debug_assert!(y_a * y_b < 0.0);
    debug_assert!(y_a < y_b);

    let n_half = (((b - a) / epsilon).log2().ceil() - 1.0).max(0.0) as usize;
    let n_max = n_half + N_0;
    let k_1 = 0.2 / (b - a);

    let mut scaled_epsilon = epsilon * (1u64 << n_max) as f64;

    while b - a > 2.0 * epsilon {
        let x_half = 0.5 * (a + b);
        let r = scaled_epsilon - 0.5 * (b - a);
        let x_f = (y_b * a - y_a * b) / (y_b - y_a);
        let sigma = x_half - x_f;
        let delta = k_1 * (b - a) * (b - a);
        let x_t = if delta <= sigma.abs() {
            x_f + delta.copysign(sigma)
        } else {
            x_half
        };
        let x_itp = if (x_t - x_half).abs() <= r {
            x_t
        } else {
            x_half - r.copysign(sigma)
        };
        let y_itp = f(x_itp);
        if y_itp > 0.0 {
            b = x_itp;
            y_b = y_itp;
        } else if y_itp < 0.0 {
            a = x_itp;
            y_a = y_itp;
        } else {
            return x_itp;
        }
        scaled_epsilon *= 0.5;
    }

    (a + b) * 0.5
}
