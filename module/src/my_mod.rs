// mod my_mod;

fn private_function() {
    println!("called my_mod::private_function()");
}

pub fn function() {
    println!("called my_mod::function()");
}

pub fn indirect_access() {
    print!("called my_mod::indirect_access(), that\n>");
    private_function();
}

pub mod nested {
    pub fn function() {
        println!("called my_mod::nested::function()");
    }

    fn private_function() {
        println!("called my_mod::nested::private_function()");
    }

    pub(in crate::my_mod) fn public_function_in_my_mod() {
        println!("called my_mod::nested::public_function_in_my_mod(), that\n>");
    }

    pub(self) fn public_function_in_nested() {
        println!("called my_mod::nested::public_function_in_nested()");
    }

    pub(super) fn public_function_in_super_mod() {
        println!("called my_mod::nested::public_function_in_super_mod()")
    }
}

pub fn call_public_function_in_my_mod() {
    print!("called my_mod::call_public_function_in_my_mod(), that\n>");
    nested::public_function_in_my_mod();
    print!("> ");
    nested::public_function_in_super_mod();
}

pub(crate) fn public_function_in_crate() {
    println!("called my_mod::public_function_in_crate()");
}

mod private_nested {
    pub fn function() {
        println!("called my_mod::private_nested::function()")
    }
}
