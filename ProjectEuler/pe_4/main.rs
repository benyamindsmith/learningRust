
fn main(){

    let mut largest_pallendrome = 0;

    for i in 100..999{
        for j in 100..999{
            if (i * j).to_string().chars().rev().collect::<String>() == (i * j).to_string(){
                if i*j > largest_pallendrome{
                    largest_pallendrome = i*j;
                }else{
                    continue;
                }
            }  
        }
    }

    println!("{}",largest_pallendrome);
}
