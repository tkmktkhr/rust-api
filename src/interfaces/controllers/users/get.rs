use crate::interfaces::controllers::called_log;
use crate::interfaces::controllers::Controller;
// How to Use
// let user_controller = GetUsersController {
//   name: String::from("User")
// };
// user_controller.called_log()

// class without func
pub struct GetUsersController {
    pub name: String,
}

// methods impl in class
impl Controller for GetUsersController {
    fn log(&self) {
        called_log(&self.name)
    }
}
