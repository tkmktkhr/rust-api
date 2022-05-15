pub fn called_log(name: &String) {
    println!("{}Controller is called.", name)
}

// abstract for class
pub trait Controller {
    fn log(&self);
    fn find_one_by_id(id: u32);
}

pub mod users;
