pub fn matching_with_options() {
    // 1. we can match the variants of option enum with match:
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // 2. we can use '_' placeholder to give a default operation to the unchecked matches:
    // roll a dice and if 3 comes, we have to stop, else move 1 step:
    let dice = 3;
    roll_dice(dice);

    // 3. if let binding: using if let we can specify the case where we want it to do something else ignore all other cases:
    // taking example 1 -> plus_one() : we can use if let to only perform when there is a valid value in the param passed:
    let five = Some(5);
    let six = if let Some(val) = five {             // here if let allows us to define the case in which we want to have a action and rest we can ignore.
        val+1
    } else {
        0
    };


    // taking example 2 -> dice_roll() : we can use if let to only perform stop for '3' and move for all rest:
    let dice = 3;
    if let 3 = dice {
        println!("STOP");
    } else {
        println!("MOVE");
    }
}

// 1. funtion to add 1 if the input value is valid:
fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(val) => Some(val + 1),
        None => None,
    }
}

// 2. function to check dice and if 3 is there stop, else keep moving:
fn roll_dice(dice: i32) {
    match dice {
        3 => println!("stop"),
        _ => println!("move"),
    }
}
