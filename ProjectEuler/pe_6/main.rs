fn sum_squares(vals:Vec<i32>) -> i32{

    let mut squares = vec![];

    for i in vals.into_iter(){
        squares.push(i.pow(2));
    }   

    return squares.iter().sum();
}

fn square_sum(vals: Vec<i32>) -> i32{

    let mut sum:i32 = 0;

    for i in vals.into_iter(){
        sum += i;
    }

    return sum.pow(2);
} 


fn main(){

    let dt: Vec<_> = (1..101).collect();


    print!(
    "{}",
    square_sum(dt.clone())-sum_squares(dt)
    );
}
