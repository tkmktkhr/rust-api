use crate::interfaces::controllers::called_log;
use crate::interfaces::controllers::Controller;

// class without func
pub struct GetUsersController {
    pub name: String,
    // UseCase
}

// methods impl in class
impl Controller for GetUsersController {
    fn log(&self) {
        called_log(&self.name)
    }

    fn find_one_by_id(id: u32) {
        // var inputData = new UserCreateInputData(userName);
        // userCreateUseCase.Handle(inputData);

        let input_data = FindUserInputData { id };
        let output_data = FindUserInteractor::get_user_by_id(input_data);
        output_data
        // let output_data = User{}
        //
    }
}

// DTO<Input>
pub struct FindUserInputData {
    pub id: u32,
}
// DTO<Output>
pub struct FindUserOutputData {
    pub id: u32,
}

// Use Case implementation
struct FindUserInteractor {}

impl FindUserInteractor {
    fn get_user_by_id(input: FindUserInputData) {
        let output = FindUserOutputData { id: input.id };
        output;
    }
}

// For View
struct FindUserPresenter {
    pub id: u32,
}
