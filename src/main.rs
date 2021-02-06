fn main() {

    /*
     * Example one
     */
    println!("{}", "Example One");
    let input_data = String::from("--test input");
    let two_dashes = "--";
    //                    I     O                 E     I
    let x: nom::IResult<&str, &str, nom::error::Error<&str>> = nom::bytes::complete::tag_no_case(two_dashes)(&input_data);
    println!("{}", "This first example works! I've assigned the output to a variable, to illustrate that I understand the type the variable needs to have.");
    println!("{}", "tag_no_case returns a function that itself returns a nom::IResult<I, O, E>, and by looking into the docs, I can see what the I, O and E types should be, and by explicitly type-annotating that variable, all is fairly cheery, and educational.");
    println!("{:?}", x);

    /*
     * Example two
     */
    println!("{}", "Example Two");
    let input_data = String::from("faulty input");
    let two_dashes = "--";
    let x: nom::IResult<&str, &str, nom::error::Error<&str>> = nom::bytes::complete::tag_no_case(two_dashes)(&input_data);
    println!("{}", "This second example won't work, because I've fed it data that doesn't match the tag.");
    println!("{:?}", x);


    /*
     * Example three
     */
    println!("{}", "Example Three");
    let input_data = String::from("--hat");
    let mut x = nom::sequence::pair(nom::bytes::complete::tag_no_case(two_dashes), nom::bytes::complete::tag_no_case("hat"));
    let y: nom::IResult<&str, (&str, &str), nom::error::Error<&str>> = x(&input_data);
    println!("{}", "This example uses two tag parsers, combined in a 'pair' parser. This time the pair is assigned to a variable (x), just for fun, and the variable is used to call the function to return the iResult into y. ");
    println!("{}", "Noting the two output types, one for each parser in the pair, makes me realise that a parser could return anything, which might be useful to output a 'translation' of something matched. Maybe an enum variant instead of a str that will need to be further compared to 'read' what it says.");
    println!("{}", "Note also that the variable x that holds the parser function needs to be declared mutable. I don't really know why.");
    println!("{:?}", y);

}
