fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => println!("dice: 3"),
        7 => println!("dice: 7"),
        other => println!("dice: {other}"),
        _ => todo!("Impossible!"),
        _ => ()
    }
}
