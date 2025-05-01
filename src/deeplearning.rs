use create::matrix;

use crate::{activation_func, matrix};

pub fn rnn(input_seq:&Vec<Vec<f64>>,initial_hid_state:&Vec<f64>,wx:&Vec<Vec<f64>>,wh:&Vec<Vec<f64>>,bias:&Vec<f64>){

    for(_,row in input_seq.iter().enumerate()) {
       
        let wx_with_x=matrix::dotproduct_1d_1d(wx,x);
        let wh_with_h=matrix::dotproduct_1d_1d(wh, t);
       let mut h=vec![0i32;bias.len()];
        for i in bias.len() {
                 h=activation_func::tanh(w)
        }

       
    }
}