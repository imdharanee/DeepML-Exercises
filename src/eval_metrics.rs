
pub fn accuracy(y_true:&Vec<i32>,y_pred:&Vec<i32>) ->f64
{
      
       let tp=true_positive(y_true, y_pred);
       tp as f64/y_pred.len() as f64
}
pub fn precision(y_pred:&Vec<i32>,y_true:&Vec<i32>)->f64 {
   let tp=true_positive(y_true, y_pred);
   let fp=false_positive(y_true, y_pred);

   if tp+fp==0 {
       0.0
   }
   else {
       tp as f64/(tp+fp)as f64
   }
}
pub fn recall(y_pred:&Vec<i32>,y_true:&Vec<i32>)->f64 {

   let tp=true_positive(y_true, y_pred);
   let f_n=false_negative(y_true, y_pred);

   if tp+f_n==0 {
      0.0
   }
   else {
      tp as f64/(tp+f_n) as f64
   }
}

pub fn fscore(y_pred:&Vec<i32>,y_true:&Vec<i32>)->f64 {

}

pub fn true_positive(y_true:&Vec<i32>,y_pred:&Vec<i32>) ->i32 {
   let mut count=0;
   for i in 0..y_true.len() {
       if y_true[i]==1 && y_pred[i]==1 {
           count+=1;
       }
   }
   count
}
pub fn true_negative(y_true:&Vec<i32>,y_pred:&Vec<i32>) ->i32 {
    
    let mut count=0;
   for i in 0..y_true.len() {
       if y_true[i]==1 && y_pred[i]==0 {
           count+=1;
       }
   }
   count
}
pub fn false_positive(y_true:&Vec<i32>,y_pred:&Vec<i32>) ->i32 {

    let mut count=0;
    for i in 0..y_true.len() {
        if y_true[i]==0 && y_pred[i]==1 {
            count+=1;
        }
    }
    count
}
pub fn false_negative(y_true:&Vec<i32>,y_pred:&Vec<i32>) ->i32 {
    let mut count=0;
    for i in 0..y_true.len() {
        if y_true[i]==0 && y_pred[i]==0 {
            count+=1;
        }
    }
    count
}


