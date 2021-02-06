fn main() {

    let input_data = String::from("--test input");
    let two_dashes = "--";
    let x: nom::IResult<&str, &str, nom::error::Error<&str>> = nom::bytes::complete::tag_no_case(two_dashes)(&input_data);
    println!("{:?}", x);

}
