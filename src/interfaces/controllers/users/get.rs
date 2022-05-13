
use crate::interfaces::controllers::called_log;
use crate::interfaces::controllers::Controller;
// How to Use
// let user_controller = UserController {
//   name: String::from("User")
// };
// user_controller.called_log()

// class without func
struct GetUsersController {
  name: String
}

// methods impl in class
impl Controller for GetUsersController {
  fn log(&self) {
    called_log(&self.name)
  }
}

