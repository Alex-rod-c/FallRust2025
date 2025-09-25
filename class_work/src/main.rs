fn sun(total: &mut i32, low: i32, high: i32){
    *total_sum = 0;
    for i in low..=high{
        *total_sum += i;
    }
}

fn main()
{
    let mup total_sum = 0
    sum(&mut total_sum, 0, 100);
    println!("Total sum from 0 to 100 is: {}", total_sum)
}


