enum choices {
    rock,
    paper,
    scissors
}

enum winner {
    comp,
    user,
    draw
}

fn get_winner(user_pick: choices, comp_pick: choices) -> winner {
    match (user_pick, comp_pick) {
        (choices::paper, choices::rock) => winner::user,
        (choices::paper, choices::scissors) => winner::comp,
        (choices::paper, choices::paper) => winner::draw,

        (choices::rock, choices::paper) => winner::comp,
        (choices::rock, choices::scissors) => winner::user,
        (choices::rock, choices::rock) => winner::draw,

        (choices::scissors, choices::paper) => winner::user,
        (choices::scissors, choices::rock) => winner::comp,
        (choices::scissors, choices::scissors) => winner::draw
    }
}

fn main() {
    let user_pick = choices::paper;
    let comp_pick = choices::scissors;

    let winner = get_winner(user_pick, comp_pick);

    match winner {
        winner::comp => println!("Computer won!"),
        winner::user => println!("User won!"),
        winner::draw => println!("Draw!")
    }
}
