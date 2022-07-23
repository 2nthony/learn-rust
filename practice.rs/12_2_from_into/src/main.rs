use std::{fs, io, num};

fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
}

fn a1() {
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    let i3: u32 = 'a' as u32;
    let i3: u32 = 'a'.into();
    println!("{i3}");

    let s: String = 'a'.into();
    let s: String = String::from('a');
    println!("{s}");

    println!("1 ok")
}

fn a2() {
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(value: i32) -> Self {
            Number { value }
        }
    }

    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30.into();
    assert_eq!(num.value, 30);

    println!("2 ok")
}

fn a3() {
    enum CliError {
        IoError(io::Error),
        ParseError(num::ParseIntError),
    }

    impl From<io::Error> for CliError {
        fn from(error: io::Error) -> Self {
            CliError::IoError(error)
        }
    }

    impl From<num::ParseIntError> for CliError {
        fn from(error: num::ParseIntError) -> Self {
            CliError::ParseError(error)
        }
    }

    fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
        let contents = fs::read_to_string(&file_name)?;

        let num: i32 = contents.trim().parse()?;

        Ok(num)
    }

    open_and_parse_file("./main.rs");

    println!("3 ok")
}

fn a4() {
    let n: i16 = 256;
    let n: i8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };

    println!("4 ok")
}

fn a5() {
    #[derive(Debug, PartialEq, Eq)]
    struct EvenNum(i32);

    impl TryFrom<i32> for EvenNum {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNum(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));

    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("5 ok")
}
