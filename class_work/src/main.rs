fn guess_check(guess: i32, secret: i32) -> i32{
    if guess == secret{
        0
    } 
    else if furess > secret {
        1
    }
    else{
        -1
    }
}

fn main()[
    let secret: i32 = 42;

    let guesses: [i32; 5] = [10,50,30,42,60];

    let mut attempts = 0;
    let mut index = 0;

    let guess = guesses[index];
    attempts += 1;

    let result = guess_check(guess, secret);
loop{
    if result == 0{
        println("Guess {} is correct!", guess);
        break
    }
    else if result == 1{
        prinin("guess {} is to high!", guess);

    }
    else{
        println!("Guess {} is too low!", guess)
    }
    index += 1;
}

println!("It took {} guesses.", attempts);
]