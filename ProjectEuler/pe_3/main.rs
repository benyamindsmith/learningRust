fn is_prime(num: i64) -> bool{

    if num <= 1{
        return false;
    } else if  num <= 3{
        return true;
    } else if num % 2 == 0 || num % 3 == 0{
        return false;
    }

    let mut i: i64 = 5;

    while i*i <= num{
        if num % i == 0 || num % (i+2) == 0{
            return false;
        }

        i += 6
    }

    return true;
}

fn main(){
    let num: i64 = 600851475143;
    
    for i in (2..=num/2).rev() {
       if num % i == 0{
           if is_prime(i){
            println!("{}",i);
            break;
           }
       } 
    }
}
