fn main(){
    let limit: i32 = 4000000; 
    let mut fib_1: i32 = 0;
    let mut fib_2: i32 = 1;
    let mut fib_3: i32 = 0;
    let mut total: i32 = 0;

    while fib_3 <= limit{
        fib_3= fib_1 + fib_2;
    
        if fib_3 % 2 == 0{
            total += fib_3
        }
        
        fib_1 = fib_2;
        fib_2 = fib_3;

    }

    println!("{}",total);
}
