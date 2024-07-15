// @generated automatically by Diesel CLI.

diesel::table! {
    teachers (id) {
        id -> Nullable<Integer>,
        payfoll -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}
