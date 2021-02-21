/**
The `Attributes` struct holds parameters that will affect how the main program runs. Upon creating an instance of the struct with `new()`, it will contain
default values that can be overwritten with command-line attributes.
*/
pub struct Attributes {
    /// Hat is not a real struct element, but it takes a `String`, as an example. Really this is only here to test the doc comment.
    pub hat: String,
    pub bat: i32,
    pub cat: String,
    /// HasNoData does not accept a parameter on the command line. Rather, its presence or absence is recorded, as true or false.
    pub has_no_data: bool,
    pub has_a_number: i32,
    /// Created properly is *not* public. This is to avoid creation of an instance of the struct without using the associated methods.
    _dont_created_directly_use_new: (),
}
impl Attributes {
 
    pub fn new() -> Self {
        Attributes{
            hat: String::from("basic"),
            bat: 500,
            cat: String::from("tabby"),
            has_no_data: false,
            has_a_number: 0,
            _dont_created_directly_use_new: ()
        }
    }

    pub fn set_hat(&mut self, new_hat_value: String){
        self.hat = new_hat_value;
    }

    pub fn set_bat(&mut self, new_bat_value: i32){
        self.bat = new_bat_value;
    }

    pub fn set_cat(&mut self, new_cat_value: String){
        self.cat = new_cat_value;
    }

    pub fn set_has_no_data(&mut self, new_has_no_data_value: bool){
        self.has_no_data = new_has_no_data_value;
    }

    pub fn set_has_a_number(&mut self, new_has_a_number_value: i32){
        self.has_a_number = new_has_a_number_value;
    }

}


mod tests {

    use super::Attributes;

    #[test]
    fn test_create_attributes(){
        let _a = Attributes{
            hat: String::from("trilby"),
            cat: String::from("siamese"),
            bat: 50,
            has_a_number: 67,
            has_no_data: true,
            _dont_created_directly_use_new: (),
        };
    }

    #[test]
    fn test_create_attributes_by_new(){
        let a = Attributes::new();
    }
}
