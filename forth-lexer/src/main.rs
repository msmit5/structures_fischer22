mod lexer;
fn main() {
    println!("Hello, world!");

    let mut lxr = lexer::Lexer::new(String::from("/home/matt/classwork/Fall-Senior/structures/programs/forth-lexer/test.fth")).unwrap();

    lxr.do_lex();
    lxr.print();
}
