use codegen_proc_macro::codegen;

codegen!("test.json");

fn main() {
    my_generated_module::fun1();
    my_generated_module::fun2();
    my_generated_module::fun3();
}
