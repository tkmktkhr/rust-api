table! {
    users (id) {
        id -> Unsigned<Bigint>,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
    }
}
