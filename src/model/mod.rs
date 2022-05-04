mod regular;

use speedy::{Readable, Writable};

use crate::util::solve_itp;

#[derive(PartialEq, Debug, Readable, Writable)]
pub struct Player {
    pub id: u64,
    pub name: String,
    pub mu: f64,
    pub mu_pi: f64,
    pub sigma: f64,
    pub delta: f64,
    pub perfs: Vec<f64>,
    pub weights: Vec<f64>,
}

impl Player {
    pub fn new(id: u64, name: String, mu_init: f64, sigma_init: f64) -> Player {
        Player {
            id,
            name,
            mu: mu_init,
            mu_pi: 0.0,
            sigma: sigma_init,
            delta: 0.0,
            perfs: vec![mu_init],
            weights: vec![1.0 / (sigma_init * sigma_init)],
        }
    }

    pub fn diffuse(&mut self, rho: f64, beta_squared: f64, gamma_squared: f64) {
        let kappa = 1.0 / (1.0 + gamma_squared / (self.sigma * self.sigma));
        let kappa_rho = kappa.powf(rho);
        let w_g = kappa_rho * self.weights[0];
        let w_l = (1.0 - kappa_rho) * self.weights.iter().sum::<f64>();
        self.perfs[0] = (w_g * self.perfs[0] + w_l * self.mu) / (w_g + w_l);
        self.weights[0] = kappa * (w_g + w_l);
        let kappa_rho = kappa_rho * kappa;
        self.weights
            .iter_mut()
            .skip(1)
            .for_each(|w| *w *= kappa_rho);
        self.sigma /= kappa.sqrt();
        self.mu_pi = self.mu;
        self.delta = ((self.sigma * self.sigma) + beta_squared).sqrt();
    }

    pub fn update(
        &mut self,
        beta_squared: f64,
        player_data: &[(f64, f64)],
        (lo, hi): (usize, usize),
    ) {
        // COEFF = pi / sqrt(3)
        const COEFF: f64 = 1.8137993642342178;
        const SOLVE_BOUND: (f64, f64) = (-10000.0, 10000.0);
        const SOLVE_EPSILON: f64 = 1e-7;

        let beta = beta_squared.sqrt();

        let f = |x: f64| {
            player_data
                .iter()
                .skip(lo - 1)
                .map(|&(delta, mu_pi)| ((0.5 * COEFF * (x - mu_pi) / delta).tanh() - 1.0) / delta)
                .sum::<f64>()
                + player_data
                    .iter()
                    .take(hi)
                    .map(|&(delta, mu_pi)| {
                        ((0.5 * COEFF * (x - mu_pi) / delta).tanh() + 1.0) / delta
                    })
                    .sum::<f64>()
        };

        self.perfs.push(solve_itp(f, SOLVE_BOUND, SOLVE_EPSILON));
        self.weights.push(1.0 / beta_squared);

        let g = |x: f64| {
            self.weights[0] * (x - self.perfs[0])
                + self
                    .perfs
                    .iter()
                    .skip(1)
                    .zip(self.weights.iter().skip(1))
                    .map(|(&perf, &weight)| {
                        COEFF * weight * beta * (0.5 * COEFF * (x - perf) / beta).tanh()
                    })
                    .sum::<f64>()
        };

        self.mu = solve_itp(g, SOLVE_BOUND, SOLVE_EPSILON);
    }
}
