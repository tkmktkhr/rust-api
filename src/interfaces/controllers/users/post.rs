use crate::{
    interfaces::controllers::called_log,
    use_cases::users::create_user,
    use_cases::users::create_user::{CreateUserInteractor, CreateUserOutputData},
};

// kind of class without func
pub struct PostUsersController {
    pub name: String,
    // UseCase
}

// トレイト(trait)とは任意の型となりうるSelfに対して定義されたメソッドの集合のこと
pub trait PostUsersControllerTrait {
    fn new(name: String) -> Self;
    fn create_user(&self, id: u32) -> CreateUserOutputData; // remove here but it will be error in Controller because there is no abstract concept in Rust.
}

// methods impl in struct
impl PostUsersControllerTrait for PostUsersController {
    fn new(name: String) -> Self {
        called_log(&name);

        Self {
            name: name.to_owned(),
        }
    }

    fn create_user(&self, id: u32) -> CreateUserOutputData {
        // var inputData = new UserCreateInputData(userName);
        // userCreateUseCase.Handle(inputData);

        let input_data = create_user::CreateUserInputData { id };
        let output_data = create_user::CreateUserInteractor::create(input_data);
        // if there is no user, return Not found.
        // let output_data = match output_data {
        //   Ok(user) => user,
        //   Err(error) => {
        //       // Error
        //       panic!("There was a problem opening the file: {:?}", error)
        //   },
        // };
        output_data
    }
}
