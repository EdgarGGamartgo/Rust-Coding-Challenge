# Code Generation Macro
This Rust macro allows you to generate Rust code based on a JSON configuration file. The generated code includes functions specified in the JSON file.

The codegen_proc_macro is a Rust procedural macro designed to facilitate code generation based on a JSON configuration file 'test.json'. By leveraging the serde and syn libraries, the macro parses a specified JSON file containing a list of method names. It then generates Rust code, defining functions with the specified names. The generated code is encapsulated within a module for better organization. The generated Rust code can seamlessly integrate into the main program where
currently I'm printing logs to the terminal based on the defined functions.

# example usage

1. Please download the repo and from root directory run the following commands to build and run your Rust program

cargo build
cargo run

This will generate the functions and execute them in the main function. The functions are defined in ´test.json´