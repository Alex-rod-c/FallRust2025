enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}

struct Inventory {
    fruit: Vec<Fruit>,
}

impl Inventory {
    fn available_fruits(&self) {
        for f in &self.fruit {
            Inventory::tell_me_joke(f);
        }
    }

    fn tell_me_joke(fruit: &Fruit) { 
        match fruit{
            Fruit::Apple(s) => println!("{}", s),
            Fruit::Banana(s) => println!("{}", s),
            Fruit::Tomato(s) => println!("{}", s), 
        }
        }
}

fn main(){
    let a = "An apple and oranges but it was a peach.".to_string();
    let b = "A banana peel made me slip.".to_string();
    let t = "A tomato a day keeps the doctor away.".to_string();
    let fruits = vec![Fruit::Banana(b),Fruit::Apple(a),Fruit::Tomato(t)];
    let grocery_store = Inventory {
        fruit:fruits,
    };
   grocery_store.available_fruits();
}