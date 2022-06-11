use crate::{
    interfaces::controllers::called_log, use_cases::users::find_user,
    use_cases::users::find_user::FindUserOutputData,
};

// kind of class without func
pub struct GetUsersController {
    pub name: String,
}

// Traits are similar to a feature often called interfaces in other languages, although with some differences.
pub trait GetUsersControllerTrait {
    fn new(name: String) -> Self;
    fn find_one_by_id(&self, id: u32) -> FindUserOutputData; // remove here but it will be error in Controller because there is no abstract concept in Rust.
}

// methods impl in struct
impl GetUsersControllerTrait for GetUsersController {
    fn new(name: String) -> Self {
        called_log(&name);

        Self { name }
    }

    fn find_one_by_id(&self, id: u32) -> FindUserOutputData {
        // NOTE Clean Architecture Sample.
        // var inputData = new UserCreateInputData(userName);
        // userCreateUseCase.Handle(inputData);

        let input_data = find_user::FindUserInputData { id };
        let output_data = find_user::FindUserInteractor::get_user_by_id(input_data);
        // if there is no user, return Not found.
        // let output_data = match output_data {
        //     Ok(user) => user,
        //     Err(error) => {
        //         // Error
        //         panic!("There was a problem opening the file: {:?}", error)
        //     }
        // };
        output_data
    }
}
