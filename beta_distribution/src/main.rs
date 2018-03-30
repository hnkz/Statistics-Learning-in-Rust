
mod beta;

fn main() {
    let mut beta = beta::BetaDist::new(10f64, 10f64);

    eprintln!("{:?}\n", beta);

    let times = 100;
    for i in (0..(times+1)) {
        let lambda = (i as f64) / (times as f64);
        println!("Pr(λ = {:^5})= {}", lambda, beta.get_prob(lambda));
    }
    // for i in (0..(times+1)) {
    //     let lambda = (i as f64) / (times as f64);
    //     println!("{},{}", lambda, beta.get_prob(lambda));
    // }
}
