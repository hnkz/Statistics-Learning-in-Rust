
#[derive(Debug)]
pub struct BetaDist {
    alpha: f64,
    beta: f64,
    gamma: f64,
}

impl BetaDist {
    pub fn new(alpha: f64, beta: f64) -> BetaDist {
        BetaDist {
            alpha: alpha,
            beta: beta,
            gamma: BetaDist::calculate_beta_func(&alpha, &beta)
        }
    }

    pub fn get_prob(&self, lambda: f64) -> f64 {
        self.gamma * lambda.powf(self.alpha-1f64) * (1f64-lambda).powf(self.beta-1f64)
    }

    fn calculate_beta_func(alpha: &f64, beta: &f64) -> f64 {
        let times = 1000000f64;
        let delta = 1f64 / times;

        let mut t = 0f64;
        let mut sum = 0f64;
        for i in 0..(times as i32) {
            sum += (t.powf(alpha-1f64)*(1f64 - t).powf(beta-1f64)) / times;
            t += delta;
        }
        
        sum
    }
}