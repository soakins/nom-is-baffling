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

}
