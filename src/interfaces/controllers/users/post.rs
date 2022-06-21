use crate::{
    interfaces::controllers::called_log, use_cases::users::create_user,
    use_cases::users::create_user::CreateUserOutputData,
};

// kind of class without func
pub struct PostUsersController {
    pub name: String,
    // UseCase
}

pub trait PostUsersControllerTrait {
    fn new(name: String) -> Self;
    fn create_user(&self, id: u32) -> CreateUserOutputData; // remove here but it will be error in Controller because there is no abstract concept in Rust.
}

// methods impl in struct
impl PostUsersControllerTrait for PostUsersController {
    fn new(name: String) -> Self {
        called_log(&name);

        Self { name }
    }

    fn create_user(&self, id: u32) -> CreateUserOutputData {
        let input_data = create_user::CreateUserInputData { id };
        return create_user::CreateUserInteractor::create(input_data);
    }
}
