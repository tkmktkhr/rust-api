use crate::use_cases::users::find_user::FindUserOutputData;

pub fn called_log(name: &String) {
    println!("{}Controller is called.", name)
}

pub trait Controller {
    fn new(name: String) -> Self;
    fn find_one_by_id(id: u32) -> FindUserOutputData; // remove here but it will be error in Controller because there is no abstract concept in Rust.
}

pub mod users;
