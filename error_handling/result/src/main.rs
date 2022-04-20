use std::num::ParseIntError;

fn main() {
    // basic();
    // result_map();
    early_return();
}

fn basic() {
    fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
        let first_number = first_number_str.parse::<i32>().unwrap();
        let second_number = second_number_str.parse::<i32>().unwrap();

        first_number * second_number
    }

    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}

fn result_map() {
    type AliasedResult<T> = Result<T, ParseIntError>;

    fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        match first_number_str.parse::<i32>() {
            Ok(first_number) => match second_number_str.parse::<i32>() {
                Ok(second_number) => Ok(first_number * second_number),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    fn multiply_combination(
        first_number_str: &str,
        second_number_str: &str,
    ) -> Result<i32, ParseIntError> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply_combination("t", "2");
    print(tt);
}

fn early_return() {
    fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        let first_number = match first_number_str.parse::<i32>() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        let second_number = match second_number_str.parse::<i32>() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        Ok(first_number * second_number)
    }

    fn multiply_2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        let first_number = first_number_str.parse::<i32>()?;
        let second_number = second_number_str.parse::<i32>()?;

        Ok(first_number * second_number)
    }

    // deprecated try!
    // fn multiply_try(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    //     let first_number = try!(first_number_str.parse::<i32>());
    //     let second_number = try!(second_number_str.parse::<i32>());

    //     Ok(first_number * second_number)
    // }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    print(multiply("10", "2"));
    print(multiply_2("t", "2"));
}
