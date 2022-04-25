#[derive(Debug, Clone)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub mu: f64,
    pub mu_pi: f64,
    pub sigma: f64,
    pub delta: f64,
    pub perfs: Vec<f64>,
    pub weights: Vec<f64>
}