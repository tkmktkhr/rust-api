use crate::interfaces::controllers::called_log;
// use crate::interfaces::controllers::Controller;
use crate::use_cases::users::find_user;
use crate::use_cases::users::find_user::FindUserOutputData;

// class without func
pub struct GetUsersController {
    pub name: String,
    // UseCase
}

// methods impl in class
impl GetUsersControllerTrait for GetUsersController {
    fn new(name: String) -> Self {
        called_log(&name);

        Self {
            name: name.to_owned(),
        }
    }

    fn find_one_by_id(&self, id: u32) -> FindUserOutputData {
        // var inputData = new UserCreateInputData(userName);
        // userCreateUseCase.Handle(inputData);

        let input_data = find_user::FindUserInputData { id };
        let output_data = find_user::FindUserInteractor::get_user_by_id(input_data);
        output_data
    }
}

// トレイト(trait)とは任意の型となりうるSelfに対して定義されたメソッドの集合のこと
pub trait GetUsersControllerTrait {
    fn new(name: String) -> Self;
    fn find_one_by_id(&self, id: u32) -> FindUserOutputData; // remove here but it will be error in Controller because there is no abstract concept in Rust.
}
