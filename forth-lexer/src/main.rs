mod lexer;
fn main() {
    println!("Hello, world!");

    let lxr = lexer::Lexer::new(String::from("/home/matt/classwork/Fall-Senior/structures/programs/forth-lexer/test.fth"));
    lxr.unwrap().do_lex();
}
