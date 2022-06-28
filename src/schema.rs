table! {
    users (id) {
        id -> Unsigned<Integer>,
        first_name -> Text,
        last_name -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}

// table! {
//   users (id) {
//       id -> Unsigned<Integer>,
//       first_name -> Varchar,
//       last_name -> Nullable<Varchar>,
//       email -> Nullable<Varchar>,
//   }
// }
