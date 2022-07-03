#[derive(Debug)]
struct Lawn {
    length: i32,
    width: i32
}

impl Lawn {
    fn show_size(self: &Self) {
        println!("Size is: {}", self.width * self.length)
    }
}

fn main() {
    let mut all_lawns: Vec<Lawn> = vec![];

    let first_lawn = Lawn {length: 20, width: 50};
    let second_lawn = Lawn {length: 5, width: 2};

    all_lawns.push(first_lawn);
    all_lawns.push(second_lawn);
    
    details_shower(&all_lawns);
}

fn details_shower(lawns: &Vec<Lawn>) {
    for lawn in lawns {
        lawn.show_size();   
    }
}