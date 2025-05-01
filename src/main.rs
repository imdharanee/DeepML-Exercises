use std::vec;
mod matrix;

mod activation_func;
mod eval_metrics;
mod deeplearning;

 




 
 


fn linear_kernel_func(x:&Vec<i32>,y:&Vec<i32>) ->i32
{
       let mut s=0;
       for i in 0..x.len() {
            s+=x[i]*y[i];
       }
       s
}

 fn main() {
     let matrix:Vec<Vec<i32>>=vec![vec![1,2],vec![2,4]];
     
     let  vector:Vec<i32>=vec![1,2];
     
     let res=matrix::perform_dotproduct_with_matrix_and_1darry(&matrix,&vector);
     println!("DotProduct of matrix and 1-d array is done");

     println!("{},{}",res[0],res[1]);

     

     let theta=matrix::linear_normal_eq(&matrix, &vector);

     

     for i in 0..theta.len()
     {
          println!("Theta value is {}",theta[i as usize]);
     }
    let z=12f64;
     let x=activation_func::sigmoid(z);
     println!("Activation function:{}",x);

     let scores:Vec<f64>= vec![1.0,2.0, 3.0];
     let probabilities = activation_func::softmax(&scores);
    println!("{:?}", probabilities);

    let y_true=vec![1,1,1,1];
    let y_pred=vec![1,1,0,0];

    println!("{}",eval_metrics::accuracy(&y_true, &y_pred));
    
    



 }