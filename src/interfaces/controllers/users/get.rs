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

    // fn find_one_by_id(id: u32) {

    // }
}
