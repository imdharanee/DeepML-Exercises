pub fn scalar_mult(matrix:&mut Vec<Vec<f64>>,scalar:i32) {
    matrix.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|num| *num *= scalar as f64);
    });

 }
 pub fn vect_to_diag(arr:&Vec<i32>)->Vec<Vec<f64>> {
    let size=arr.len();
    let mut diag=vec![vec![0f64;size];size];

   
    diag.iter_mut().enumerate().for_each(|(i, row)| {
        row.iter_mut().enumerate().for_each(|(j, col)| {
            if i == j {
                *col=  arr[i] as f64;
            }
        });
    });

    diag
   

 }
 pub fn linear_normal_eq(x:&Vec<Vec<i32>>,y:&Vec<i32>) ->Vec<i32> {

    let mut x_t=tranpose_2d(&x);

    println!("The tranpose of matrix is found successfully{}",x_t[0][0]);

    x_t=tranpose_2d(&x);

    let mut res=matrix_mult(&x, &x_t);

    inverse_2d(&mut res);

    let res2=matrix_mult(&res, &x_t);

    let fin_res=perform_dotproduct_with_matrix_and_1darry(&res2, y);

    fin_res

}

pub fn matrix_mult(x:&Vec<Vec<i32>>,y:&Vec<Vec<i32>>) ->Vec<Vec<i32>> {

    let r1=x.len();
    let c1=x[0].len();
 
    let c2=y[0].len();
 
    let mut res:Vec<Vec<i32>>=vec![vec![0;c2];r1];
    
    for i in 0..r1 {
       for j in 0..c2 {
            for k in 0..c1 {
                 res[i][j]+=x[i][k]*y[k][j];
            }
       }
    }
 
 
    res 
     
 }
 pub fn inverse_2d(x:&mut Vec<Vec<i32>>) {
       
    let temp=x[0][0];
    x[0][0]=x[1][1];
    x[1][1]=temp;
}
pub fn tranpose_2d(x:&Vec<Vec<i32>>)->Vec<Vec<i32>>
 {
      let r=x.len();
      let c=x[0].len();

      let mut x_t:Vec<Vec<i32>>=vec![vec![0;c];r];
    
      for j in 0..c {
          for i in 0..r {
              x_t[j][i]=x[i][j]
          }
      }
      x_t

}
pub fn perform_dotproduct_with_matrix_and_1darry(matrix:&Vec<Vec<i32>>,arr:&Vec<i32>)->Vec<i32> {
    
    let arr_size=arr.len();
    let mat_size=matrix.len();
    let mut res:Vec<i32>=vec![0i32;mat_size];
    
    for i in 0..mat_size 
    {
          for j in 0..arr_size
          {
                res[i]+=matrix[i][j]*arr[j];
          }
    }
    res
    
    
    
 }
 pub fn dotproduct_1d_1d(x:&Vec<f64>,y:&Vec<f64>)->f64 {
       let sum=0.0;
       for i in 0..x.len() {
            sum+=x[i] * y[i];
       }
       
 }
