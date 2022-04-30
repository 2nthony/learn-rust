use utf8_slice;

fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
    a6();
    a7();
    a8();
    a9();
    a10();
    a11();
    a12();
    example1();
}

fn a1() {
    let s: &str = "hello";
}

fn a2() {
    // let s: Box<str> = "hello world".into();
    // greetings(&s);

    let s: Box<&str> = "hello world".into();
    greetings(*s);

    fn greetings(s: &str) {
        println!("2 {}", s);
    }
}

fn a3() {
    let mut s = String::from("");
    s.push_str("hello world");
    s.push('!');

    assert_eq!(s, "hello world!");

    println!("3 success");
}

fn a4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("4 {}", s)
}

fn a5() {
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("5 success");
}

fn a6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;

    assert_eq!(s3, "hello,world!");
    println!("6 {}", s1);
}

fn a7() {
    let s = "hello, world!";

    greetings(s.to_string());
    greetings(String::from(s));

    fn greetings(s: String) {
        println!("7 {}", s);
    }
}

fn a8() {
    let s = "hello, world!".to_string();
    let s1: &str = &s;
    let s2: &str = &s[..];
    let s3: &str = &s[..s.len()];
    let s4: &str = &s.as_ref();
}

fn a9() {
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("9 What are your doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "Strin literals
                        can span multiple lines.
                        The linebreak and indentation here \
                        can be escaped too!";
    println!("9 {}", long_string);
}

fn a10() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: \\x3F \\u{211D}");

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("10 {}", quotes);

    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("10 {}", delimiter);

    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");
    println!("10 {}", long_delimiter);
}

fn a11() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1];
    assert_eq!(h, "h");
    println!("11 {}", h);

    let h1 = &s1[3..6];
    assert_eq!(h1, "中");
    println!("11 {}", h1);
}

fn a12() {
    for c in "你好，世界".chars() {
        println!("12 {}", c);
    }
}

fn example1() {
    let s = "The 🚀 gost to the 🌑!";
    let rocket = utf8_slice::slice(s, 4, 5);
    let rocket2 = &s[4..];
    println!("example1 {}", rocket);
    println!("example1 {}", rocket2);
}
