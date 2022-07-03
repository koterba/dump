enum opinion {
    good,
    bad
}

struct food_opinion {
    name: String,
    op: opinion
}

fn main() {
    let mut foods: Vec<food_opinion> = vec![];
    
    let pizza = food_opinion {
        name: String::from("pizza"),
        op: opinion::good
    };
    let fish = food_opinion {
        name: String::from("fish"),
        op: opinion::bad
    };
    
    foods.push(pizza);
    foods.push(fish);
    
    food_pretty_printer(&foods);
}

fn food_pretty_printer(foods: &Vec<food_opinion>) {
    for food in foods {
        let opinion = match food.op {
            opinion::good => String::from("good"),
            opinion::bad => String::from("bad")
        };
        println!("Food: {}, Opinion: {}",food.name, opinion)
    }
}
