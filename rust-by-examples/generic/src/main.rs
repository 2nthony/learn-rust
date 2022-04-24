mod associated_item;
mod basic;
mod bounds;
mod function;
mod implement;
mod multiple_bounds;
mod my_trait;
mod new_type;
mod phantom_type;
mod where_caluses;

fn main() {
    basic::run();
    function::run();
    implement::run();
    my_trait::run();
    bounds::run();
    bounds::run_testcase();
    multiple_bounds::run();
    where_caluses::run();
    new_type::run();
    associated_item::run();
    phantom_type::run();
    phantom_type::run_testcase();
}
