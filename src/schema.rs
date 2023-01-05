table! {
    users (id) {
        id -> Unsigned<Integer>,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        email -> Varchar,
    }
}
