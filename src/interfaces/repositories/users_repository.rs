use crate::interfaces::controllers::called_log;

pub struct UserRepository {
    pub name: String,
}

pub trait UserRepositoryTrait {
    fn new(name: String) -> Self;
    // fn find_one_by_id(&self, id: i32) -> FindUserOutputData; // remove here but it will be error in Controller because there is no abstract concept in Rust.
}

impl UserRepositoryTrait for UserRepository {
    fn new(name: String) -> Self {
        called_log(&name);

        Self { name }
    }

    // fn find_one_by_id(&self, id: i32) -> FindUserOutputData {
    //     // NOTE Clean Architecture Sample.
    //     // var inputData = new UserCreateInputData(userName);
    //     // userCreateUseCase.Handle(inputData);

    //     let input_data = find_user::FindUserInputData { id };
    //     let output_data = find_user::FindUserInteractor::get_user_by_id(input_data);
    //     // if there is no user, return Not found.
    //     // let output_data = match output_data {
    //     //     Ok(user) => user,
    //     //     Err(error) => {
    //     //         // Error
    //     //         panic!("There was a problem opening the file: {:?}", error)
    //     //     }
    //     // };
    //     output_data
    // }
}
