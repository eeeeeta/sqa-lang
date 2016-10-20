#![feature(plugin)]
#![plugin(peg_syntax_ext)]
extern crate chrono;
mod ast;
peg_file! grammar("grammar.rustpeg");
fn main() {
    println!("{:?}", grammar::quote_stmt("'filename \"test filename with \\\".png\""));
    println!("{:?}", grammar::quote_stmt("'duration-0 9h 2m 5s 200ms"));
    println!("{:?}", grammar::duration("1h 2m"));
    println!("{:?}", grammar::command(".test: load \"file.txt\" @ 20 'dur 9h 2m 'havefun"));
    println!("{:#?}", grammar::program(
        "cue 1 {
.test:        load \"filename.txt\" 'initial-volume 0;
              then after 2s load other-thing;
         };
         cue 1.2 {};"));
}
