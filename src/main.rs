mod attribute_struct;

use std::num::ParseIntError;
use attribute_struct::Attributes;

#[derive(Debug)]
enum CommandLineAttributes {
    Hat(Option<String>),
    Bat(Option<i32>),
    Cat(Option<String>),
    HasNoData,
    HasANumber(Option<i32>)
}

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

    
    /*
     * Example six
     */
    println!("{}", "Example Six");
    let whitespace_chars = " \t";

    let input_data = String::from("--hat top   --bat  pipistrelle        --cat 51");
    let two_dashes = "--";

    let is_whitespace = nom::bytes::complete::is_a(whitespace_chars);
    let is_not_whitespace = nom::bytes::complete::is_not(whitespace_chars);
    let is_not_whitespace2 = nom::bytes::complete::is_not(whitespace_chars);
    let is_optional_whitespace = nom::combinator::opt(nom::bytes::complete::is_a(whitespace_chars));
    let double_dash_tag = nom::bytes::complete::tag(two_dashes);
    let double_dashed_attribute_pair =
        nom::sequence::tuple(
            (
                double_dash_tag,          // the two hyphens
                is_not_whitespace,        // the attribute name
                is_whitespace,            // the gap after the attribute name, and before the attribute value
                is_not_whitespace2,       // the attribute value
                is_optional_whitespace    // the whitespace before the next double dash.
            )
        );
    let mut many_double_dashed_attribute_pairs = nom::multi::many0(double_dashed_attribute_pair);
    println!("{}", "Following example five, this one reads a non-whitespace piece of data following each --attribute. Note that there are two definition of is_not_whitespace(2) to avoid the problem of the variable being moved if you try to use one implementation twice in the tuple of parsers.");
    let result: nom::IResult<&str, Vec<(&str, &str, &str, &str, Option<&str>)>, nom::error::Error<&str>> = many_double_dashed_attribute_pairs(&input_data);
    println!("{:?}", result);
 
    /*
     * Example 7
     */

    println!("{}", "Example Seven");
    println!("{}", "Let's try the preceded sequence...");
    let input_data = String::from("--hat beany --cat tabby --bat 87");
    let two_dashes = "--";
    
    let x: nom::IResult<&str, Vec<&str>, nom::error::Error<&str>> =
        nom::multi::many0(
            nom::sequence::preceded(
                nom::bytes::complete::tag(two_dashes),
                nom::bytes::complete::take_until(two_dashes)
            )
        )(&input_data);

    println!("This is a bit of a new approach. I began to realise that repeating 'find some dashes, detect the attribute name, read the data, repeat' struggles if the data is missing - you wind up reading the next attribute name, complete with its two dash prefix, as if it was the missing data.  Instead, let's start by dividing things up by the double dashes. The preceded parser takes two parsers, the first it matches then discards, the second it returns. So I match and discard a TAG '--', then use take_until to read everything up to the next two dashes.");
    println!("This works, up to a point. It fails to pick up the last '--attrname value' because it doesn't end in another --. In the next example I'll see about providing an alternative ending.");
    println!("{:?}", x);

    /*
     * Example 8
     */

     println!("{}", "Example Eight");
     let input_data = String::from("--hat beany --cat tabby --bat 87        --hasnodata --hasanumber 97 ");
     let two_dashes = "--";
     
     let x_result: nom::IResult<&str, Vec<&str>, nom::error::Error<&str>> =
         nom::multi::many0(
            nom::sequence::preceded(
                nom::bytes::complete::tag(two_dashes),
                nom::branch::alt((
                   nom::bytes::complete::take_until(two_dashes),
                   nom::combinator::rest
                ))
            )
        )(&input_data);
 
    println!("This is a bit of a new approach. I began to realise that repeating 'find some dashes, detect the attribute name, read the data, repeat' struggles if the data is missing - you wind up reading the next attribute name, complete with its two dash prefix, as if it was the missing data.  Instead, let's start by dividing things up by the double dashes. The preceded parser takes two parsers, the first it matches then discards, the second it returns. So I match and discard a TAG '--', then use take_until to read everything up to the next two dashes.");
    println!("This works, up to a point. It fails to pick up the last '--attrname value' because it doesn't end in another --. In the next example I'll see about providing an alternative ending.");
    println!("{:?}", x_result);

    println!("prior to creating (directly) an Attributes struct...");
    
    /*
    You can't create an attribute_struct::Attributes directly, there's a private field that cannot be accessed outside of the... what?.... module?
    */
    
    /*
    let a = Attributes{
        hat: String::from("baseball_cap"),
        bat: 54,
        cat: String::from("witch's"),
        has_a_number: 41,
        has_no_data: false,
        _dont_created_directly_use_new: (), // created properly is intended as a mechanism to prevent us creating an Attributes, by NOT declaring it public, but it doesn't seem to work 'in crate'.
    };
    */
    println!("following creation (directly) an Attributes struct...");

    let mut a = Attributes::new();
    a.set_cat(String::from("new_cat_value"));


    /*
     * Example 9
     

    println!("{}", "Example nine will build on example eight.");
    println!("{}", "We'll whizz through the vector of results we got. Each should be an attribute name, and then some text that comprises its data. It's possible that once the attribute name is removed, there'll be nothing left, which might be appropriate for some attributes - perhaps they just switch things and take no data.");
    println!("{}", "In this bit we'll translate the textual attribute names into enum variants.");

    let hat_tag = nom::combinator::map(nom::bytes::complete::tag_no_case("hat"), |_|CommandLineAttributes::Hat(None));
    let bat_tag = nom::combinator::map(nom::bytes::complete::tag_no_case("bat"), |_|CommandLineAttributes::Bat(None));
    let cat_tag = nom::combinator::map(nom::bytes::complete::tag_no_case("cat"), |_|CommandLineAttributes::Cat(None));
    let hasnodata_tag = nom::combinator::map(nom::bytes::complete::tag_no_case("hasnodata"), |_|CommandLineAttributes::HasNoData);
    let hasanumber_tag = nom::combinator::map(nom::bytes::complete::tag_no_case("hasanumber"), |_|CommandLineAttributes::HasANumber(None));

    let mut command_line_attributes =
        nom::branch::alt((
            hat_tag,
            bat_tag,
            cat_tag,
            hasnodata_tag,
            hasanumber_tag
        ));

    if let Ok(x) = x_result {
        let vector_from_x = x.1;
        for attr_pair in vector_from_x {

            let cla_result: nom::IResult<&str, CommandLineAttributes, nom::error::Error<&str>> = command_line_attributes(attr_pair);
            
            match cla_result {
                Ok((remaining, cla)) => {
                    
                    let parse_results: std::result::Result<(&str, (&str, &str, Option<&str>, Option<&str>)), nom::Err<nom::error::Error<&str>>> =
                        nom::sequence::tuple((
                            nom::bytes::complete::is_a(whitespace_chars),                        // 1. this space will certainly be there
                            nom::bytes::complete::is_not(whitespace_chars),                      // 2. this is the data!
                            nom::combinator::opt(nom::bytes::complete::is_a(whitespace_chars)),  // 3. if there are any more characters, there must be space between, but there might not be, and in that case opt returns us a None
                            nom::combinator::opt(nom::bytes::complete::is_not(whitespace_chars)) // 4. this is the extra data that ought not to be there
                        ))(remaining);

                    // none of our examples should have extra data after the initial data. i.e.
                    // they shouldn't have a fourth part in the tuple, it should be 'None'.

                    if parse_results.is_ok() {
    
                        let parse_unwrapped = parse_results.unwrap();
                        let parsed_payload = parse_unwrapped.1;
                        let data_2 = parsed_payload.1;
                        let data_4 = parsed_payload.3;

                        if data_4.is_some() {
                            panic!("Extra unwanted data: '{}'", data_4.unwrap());
                        }

                        match cla {
                            CommandLineAttributes::Bat(b) => {
                                // bat takes an option i32;
                                let value_result: nom::IResult<&str, &str, nom::error::Error<&str>> = nom::combinator::all_consuming(nom::character::complete::digit1)(data_2);
                                match value_result {
                                    Ok(value) => {
                                        /*
                                         * We consumed all the characters, and they were digits, so we have a good chance of making a number out of them.
                                         */
                                        let value_as_number_result: Result<i32, ParseIntError> = String::from(value.1).parse();

                                        match value_as_number_result {
                                            Ok(value_as_number) => {
                                                /*
                                                 * Swap this new i32 into the 'field' in the bat enum variant
                                                 */

                                                std::mem::replace(&mut cla, CommandLineAttributes::Bat(Some(value_as_number)));
                                           }
                                            Err(errm) => {

                                            }
                                        }
                                    }
                                    Err(val_err) => {

                                    }
                                }
                            }
                            CommandLineAttributes::Cat(_) => {

                            }
                            CommandLineAttributes::Hat(_) => {

                            }
                            CommandLineAttributes::HasANumber(_) => {

                            }
                            CommandLineAttributes::HasNoData => {

                            }
                        }
                    }
                    else {   // parse_results is an Err()
                        panic!(parse_results.unwrap_err());
                    }
                }
                Err(errormessage) => println!("{}", errormessage)
            }
            
        }    
    }*/
}
