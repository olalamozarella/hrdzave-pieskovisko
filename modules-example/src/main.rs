mod separate_module;
mod folder_module;

mod inline_module {
    pub fn pub_inline_module_function() {
        println!("Hello from pub_inline_module_function");
    }

    fn priv_inline_module_function() {
        println!("Hello from priv_inline_module_function");
    }
}

// use folder_module::nested_module::pub_nested_module_function; // possible only if there is "pub mod nested_module" in nested_module/mod.rs

fn main() {
    println!("Hello from main!");
    inline_module::pub_inline_module_function();
    //inline_module::priv_inline_module_function();  // doesn't compile, function is private
    separate_module::pub_separate_module_function();
    //separate_module::priv_separate_module_function();  // doesn't compile, function is private
    folder_module::pub_folder_module_function();
    //folder_module::priv_folder_module_function();  // doesn't compile, function is private
}
