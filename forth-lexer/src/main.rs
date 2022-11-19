mod lexer;
fn main() {
    //println!("Hello, world!");

    // I do not error handle the unwrap because this is a fatal error.
    let mut lxr = lexer::Lexer::new(String::from("/home/matt/classwork/Fall-Senior/structures/programs/forth-lexer/test2.fth")).unwrap();

    lxr.do_lex();
    lxr.print();
}
