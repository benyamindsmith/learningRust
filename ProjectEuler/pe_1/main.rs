fn main(){
    let limit = 1000;
    let mut three_five_multiples_sum = 0;
    let mut i = 1;

    while i < limit {
        if i%3 == 0 || i%5 == 0 {
            three_five_multiples_sum += i;
        }
        i += 1; 
    }
    println!("{}",three_five_multiples_sum);
} 
