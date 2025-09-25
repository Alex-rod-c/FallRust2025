fn even_num(n: i32) -> bool {
    n % 2 == 0
}

fn main(){
    let numbers: [i32; 10] [3, 5, 10, 15, 22, 33, 45, 60, 71,, 99];

    for &n in numbers.iter(){
        if n % 15 == 0{
            println!("{} -> Fizzbuzz", n);
        }
        else if n % 3 == 0{
            println!("{} -> Fizz", n);
        }
        else if n % 5 == 0 {
            println("{} -> Buzz",n);
        }
        else if even_num(n){
            println("{} -> Even", n);
        }
        else{
            printin("{} -> odd", n);
        }
    }

    let mut sum = 0
    let mut i =0;
    while i < numbers.len(){
        sum += numbers[i];
        i += 1;
    }
    println!("\nsum of all numbers = {}", sum);

    let mut largest = numbers[0];
    let mut j =1;
    loop{
        if j >= numbers.len(){
            break;
        }
        if numbers[j] > largest {
            largest = numbers[j];
        }
        j += 1;
    }
    println!("Largest number = {}", largest);
}