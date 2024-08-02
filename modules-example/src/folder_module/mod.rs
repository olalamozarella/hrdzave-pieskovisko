mod nested_module;

pub fn pub_folder_module_function() {
    println!("Hello from pub_folder_module_function");
    nested_module::pub_nested_module_function();
}

fn priv_folder_module_function() {
    println!("Hello from priv_folder_module_function");
}
