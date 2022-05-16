// DTO<Input>
pub struct FindUserInputData {
    pub id: u32,
}
// DTO<Output>
pub struct FindUserOutputData {
    pub id: u32,
}

// Use Case implementation
pub struct FindUserInteractor {}

impl FindUserInteractor {
    pub fn get_user_by_id(input: FindUserInputData) {
        let output = FindUserOutputData { id: input.id };
        output;
    }
}

// For View
struct FindUserPresenter {
    pub id: u32,
}
