fn main(){
    let mut even :[i64;501] = [0;501];
    let mut odd : [i64;501] = [0;501];
    let mut prime : [i64;501] = [0;501];
    let mut index = 0u32; 
    let j = 0i32; 
    let mut usize_j = j as usize;
    let k = 0i32;
    let mut usize_k = k as usize; 
    let p = 0i32;
    let mut usize_p = p as usize;
    loop {
        if index%2 == 0 {
            // even[j] = index;
            even[usize_j] = index as i64;
            usize_j += 1;
        }
        else {
            odd[usize_k] = index as i64;
            usize_k += 1;
        }
        if is_prime(index) {
            prime[usize_p] = index as i64;
            usize_p += 1;
        }
        index +=1;
        if index == 1001 {
            break;
        }
    }

    fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }
    
        for index in 2..=(n as f64).sqrt() as u32 {
            if n % index == 0 {
                return false;
            }
        }
    
        true
    }
    println!("Even numbers :\n {:?}",even); 
    println!("Odd numbers :\n {:?}",odd);
    println!("Prime numbers :\n {:?}",prime);
}