use std::{
    fs::File,
    io::{self, Read},
    num::ParseIntError,
};

fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
    a6();
}

fn a1() {
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>();
        let n2 = n2_str.parse::<i32>();

        Ok(n1.unwrap() * n2.unwrap())
    }

    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);

    println!("1 ok")
}

fn a2() {
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;

        Ok(n1 * n2)
    }

    assert_eq!(multiply("3", "4").unwrap(), 12);

    println!("2 ok")
}

fn a3() {
    fn read_file1() -> Result<String, io::Error> {
        let f = File::open("hi.js");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_file2() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hi.js")?.read_to_string(&mut s)?;

        Ok(s)
    }

    assert_eq!(
        read_file1().unwrap_err().to_string(),
        read_file2().unwrap_err().to_string()
    );

    println!("3 ok")
}

fn a4() {
    fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
        // n_str.parse::<i32>().map(|n| n + 2)
        n_str.parse::<i32>().and_then(|n| Ok(n + 2))
    }

    assert_eq!(add_two("4").unwrap(), 6);
    println!("4 ok")
}

fn a5() {
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        match n1_str.parse::<i32>() {
            Ok(n1) => match n2_str.parse::<i32>() {
                Ok(n2) => Ok(n1 * n2),
                Err(e) => Err(e),
            },

            Err(e) => Err(e),
        }
    }

    fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        n1_str
            .parse::<i32>()
            .and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    let twenty = multiply1("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);

    println!("5 ok")
}

fn a6() {
    type Res<T> = Result<T, ParseIntError>;

    fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
    }

    fn print(result: Res<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("6 ok")
}
