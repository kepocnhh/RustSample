pub fn run() {
    const CHAPTER: u8 = 13;
    const PART: u8 = 3;
    const TITLE: &str = "Improving Our I/O Project";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _130301();
    _130302();
}

fn _130301() {
    println!("\nRemoving a clone Using an Iterator");

    let result = cloned(&["256".to_owned(), "512".to_owned()]);
    println!("cloned: {result:?}");
    let result = iterable(&["256".to_owned(), "512".to_owned()]);
    println!("iterable: {result:?}");
    let result = iterable(&["256".to_owned()]);
    println!("iterable: {result:?}");
}

fn _130302() {
    println!("\nMaking Code Clearer with Iterator Adaptors");

    let contents = "\
1: foo
2: bar
3: baz
";
    let result = fored("ba", contents);
    println!("fored: {result:?}");
    let result = iterable2("ba", contents);
    println!("iterable: {result:?}");
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

fn fored<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}

fn iterable2<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
