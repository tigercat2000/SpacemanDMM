extern crate dreammaker as dm;

use dm::Location;
use dm::lexer::*;
use dm::annotation::*;
use dm::parser::Parser;
use dm::indents::IndentProcessor;

#[test]
fn annotation_basic() {
    let code = r##"
/var/foo = bar
/datum/globals
    var/number = 7 + 5
    var/string = foo("Hello [ "world" ]")

    var/baz
    baz = "neat"

    proc/Init()
        world.log << new/obj()
"##.trim();

    let context = Default::default();
    let lexer = Lexer::new(&context, Default::default(), code.bytes().map(Ok));
    let indent = IndentProcessor::new(&context, lexer);
    let mut annotations = AnnotationTree::default();
    {
        let mut parser = Parser::new(&context, indent);
        parser.annotate_to(&mut annotations);
        parser.run();
    }
    assert!(context.print_all_errors(dm::Severity::Info));
    println!("len: {}", annotations.len());
    for each in annotations.get_location(Location {
        file: Default::default(),
        line: 9,
        column: 14,
    }) {
        println!("{:?}", each);
        for each in annotations.get_range_raw(each.0) {
            println!("    {:?}", each.1);
        }
    }
}
