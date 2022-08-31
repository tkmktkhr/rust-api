use crate::{
    interfaces::controllers::called_log, use_cases::users::find_user,
    use_cases::users::find_user::FindUserOutputData,
};

pub struct GetUsersController {
    pub name: String,
}

// Traits are similar to a feature often called interfaces in other languages, although with some differences.
pub trait GetUsersControllerTrait {
    fn new(name: String) -> Self;
    fn find_one_by_id(&self, id: i32) -> Option<FindUserOutputData>; // remove here but it will be error in Controller because there is no abstract concept in Rust.
}

impl GetUsersControllerTrait for GetUsersController {
    fn new(name: String) -> Self {
        called_log(&name);

        Self { name }
    }

    fn find_one_by_id(&self, id: i32) -> Option<FindUserOutputData> {
        // NOTE Clean Architecture Sample.
        // var inputData = new UserCreateInputData(userName);
        // userCreateUseCase.Handle(inputData);

        let input_data = find_user::FindUserInputData { id };
        let output_data = find_user::FindUserInteractor::get_user_by_id(input_data);

        // TODO return results value
        // Some(output_data)
        // let return_data = match output_data {
        //     Some(user) => user,
        //     None => None, // TODO return results as a zero value. panic!("There was not that user: {:?}", error)
        // };
        let return_data = check_user_existence(&output_data);
        return Some(return_data);
    }
}

fn check_user_existence(original: &Option<FindUserOutputData>) -> Option<&FindUserOutputData> {
    match original {
        None => None,
        Some(u) => Some(u.user),
    }
}
