use crate::interfaces::controllers::called_log;
use crate::interfaces::controllers::Controller;
use crate::use_cases::users::find_user;

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

        let input_data = find_user::FindUserInputData { id };
        let output_data = find_user::FindUserInteractor::get_user_by_id(input_data);
        output_data
        // let output_data = User{}
        //
    }
}
