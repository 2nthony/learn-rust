fn main() {
    are_you_on_linux();

    conditional_function();
}

#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("Running linux");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("Not running linux");
}

#[cfg(some_condition)]
fn conditional_function() {
    println!("Condition met");
}
