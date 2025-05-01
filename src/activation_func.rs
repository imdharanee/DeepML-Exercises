use crate::eval_metrics;

pub fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}
pub fn softmax(scores: &Vec<f64>) -> Vec<f64> {
    let exp_scores: Vec<f64> = scores.iter().map(|&score| score.exp()).collect();
    let sum_exp_scores: f64 = exp_scores.iter().sum();
    exp_scores
        .iter()
        .map(|&score| {
            let prob = score / sum_exp_scores;
            (prob * 10000.0).round() / 10000.0 // Rounding to 4 decimal places
        })
        .collect()
}
pub fn relu(input:f64)->f64 {
   if input>0.0 {
       input
   }
   else {
    0.0
   }
   
}
pub fn leaky_relu(z:f64,alpha:f64)->f64 {
       if z<0.0 {
            z*alpha
       }
       else  {
           z
       }
}
pub fn tanh(input:f64)->f64 {


}


