pub fn called_log(name: &String) {
    println!("{}Controller is called.", name)
}

// abstract for class
pub trait Controller {
    fn log(&self);
}

pub mod users;
