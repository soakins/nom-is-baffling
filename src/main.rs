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


    /*
     * Example four
     */
    println!("{}", "Example Four");
    let input_data = String::from("--hat --bat --cat");
    let two_dashes = "--";
    let double_dash_tag = nom::bytes::complete::tag(two_dashes);
    let double_dashed_word = nom::sequence::pair(double_dash_tag, nom::bytes::complete::is_not(" "));
    let mut many_double_dashed_words = nom::multi::many0(double_dashed_word);
    println!("{}", "This one defines a parser that identifies a double hyphen, pairs it with one that just reads until it finds a space, then tries repeating that pair to comsume matches 0 or more times. And it works, sort of. It consumes the --hat at the start of the input data, stopping there because it encounters the space after hat. The pair makes a tuple of the dashes and the hat, and the multi0 returns these as the first element of a vector, but the thing goes no further, because the space that stops the processing after hat is not a match for what comes next, whic is the double dash parser. Anyway, we progress.");
    let result: nom::IResult<&str, Vec<(&str, &str)>, nom::error::Error<&str>> = many_double_dashed_words(&input_data);
    /*
     * A reminder about the type of the result from many_double_dashed_words
     * nom::IResult<&str, Vec<(&str, &str)>, nom::error::Error<&str>>
     *                I            O                  E          I
     * I = input type
     * O = Output type, a Vector of tuples of the output types of the pair of parsers we're repeating
     * E = nom's Error type, that carries an I type with it.
     */
    println!("{:?}", result);

    /*
     * Example five
     */
    println!("{}", "Example Five");
    let input_data = String::from("--hat    --bat          --cat");
    let two_dashes = "--";
    let whitespace_chars = " \t";
    let is_whitespace = nom::bytes::complete::is_a(whitespace_chars);
    let is_not_whitespace = nom::bytes::complete::is_not(whitespace_chars);
    let is_optional_whitespace = nom::combinator::opt(is_whitespace);
    let double_dash_tag = nom::bytes::complete::tag(two_dashes);
    let double_dashed_word = nom::sequence::tuple((double_dash_tag, is_not_whitespace, is_optional_whitespace));
    let mut many_double_dashed_words = nom::multi::many0(double_dashed_word);
    println!("{}", "This one builds on example four. It uses a nom::sequence::tuple of parsers instead of the pair, since it now needs more that two parsers to capture each repeating 'entry'. The double dashes are recognised explicitly by a TAG, and the name of the attribute by selecting as much non-whitespace as possible. Then we consume that whitespace, IF IT'S THERE, returning None if it isn't. Then the many0 goes round again.");
    println!("{}", "At the moment, this is capturing 'command line switches', but those switches are not followed by any data. That will be next.");
    let result: nom::IResult<&str, Vec<(&str, &str, Option<&str>)>, nom::error::Error<&str>> = many_double_dashed_words(&input_data);
    /*
     * A reminder about the type of the result from many_double_dashed_words
     * nom::IResult<&str, Vec<(&str, &str)>, nom::error::Error<&str>>
     *                I            O                  E          I
     * I = input type
     * O = Output type, a Vector of tuples of the output types of the pair of parsers we're repeating
     * E = nom's Error type, that carries an I type with it.
     */
    println!("{:?}", result);
 
}
