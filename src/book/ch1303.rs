pub fn run() {
    const CHAPTER: u8 = 13;
    const PART: u8 = 3;
    const TITLE: &str = "Improving Our I/O Project";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _130301();
    todo!();
}

fn _130301() {
    println!("\nRemoving a clone Using an Iterator");

    let result = cloned(&["256".to_owned(), "512".to_owned()]);
    println!("cloned: {result:?}");
    let result = iterable(&["256".to_owned(), "512".to_owned()]);
    println!("iterable: {result:?}");
    let result = iterable(&["256".to_owned()]);
    println!("iterable: {result:?}");
    todo!();
}

fn cloned(args: &[String]) -> Result<String, String> {
    if args.len() < 2 {
        return Err("not enough arguments".to_owned());
    }
    let foo = args[0].clone();
    let bar = args[1].clone();
    return Ok(format!("foo: {foo}\nbar: {bar}"));
}

fn iterable(args: &[String]) -> Result<String, String> {
    let mut iter = args.iter();
    let foo = iter.next().ok_or("not enough arguments")?;
    let bar = iter.next().ok_or("not enough arguments")?;
    return Ok(format!("foo: {foo}\nbar: {bar}"));
}
