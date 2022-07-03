enum Food {
    Kottfarssas,
    Fakorv,
    FlyeandeJaklb
}

fn food_verify(food: Food) -> Result<bool, i32> {
	match food {
		Food::Kottfarssas => Ok(true),
		Food::Fakorv => Ok(true),
		_ => Err(0)
	}
}

fn main() {
	let favourite = Food::FlyeandeJaklb;

	if let Err(result) = food_verify(favourite) {
		println!("There was an error")
	} else {
		println!("It worked!")
	}

	//println!("{:?}", result);
}
