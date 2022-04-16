mod basic;
mod bounds;
mod function;
mod implement;
mod my_trait;

fn main() {
    basic::run();
    function::run();
    implement::run();
    my_trait::run();
    bounds::run();
    bounds::run_testcase();
}
