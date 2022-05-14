use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserStruct {
    pub id: Option<u32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

pub mod check_type {
    pub fn string(x: &Option<String>) -> std::string::String {
        match x {
            None => "".to_string(),
            Some(i) => i.to_string(),
        }
    }

    // pub fn number(x: &Option<u32>) -> std::string::String {
    //   match x {
    //       None => "".to_string(),
    //       Some(i) => i.to_string(),
    //   }
    // }
}

// Business Rules are here.
impl UserStruct {
    pub fn new(id: u32, first_name: String, last_name: String, email: String) -> UserStruct {
        UserStruct {
            id: Some(id),
            first_name: Some(first_name),
            last_name: Some(last_name),
            email: Some(email),
        }
    }

    pub fn full_name(&self) -> String {
        // let received_first_name = &self.first_name;
        let fname = check_type::string(&self.first_name);
        let lname = check_type::string(&self.last_name);
        fname + &lname.to_owned()

        // let fname: String = if received_first_name != None {
        //   Some(received_first_name)
        // } else {
        //   ""
        // };

        // let received_last_name = user.last_name;
        // let lname: String = if received_last_name != None {
        //   received_last_name
        // } else {
        //   ""
        // };

        // fname

        // Pattern1
        // let ref first = *user.first_name;
        // first.to_owned() + &user.last_name

        // Pattern2
        // user.first_name.to_owned() + &user.last_name

        // Pattern3
        // user.first_name.to_string() + &user.last_name

        // Pattern4
        // String::from(&user.first_name) + &user.last_name
    }
}
