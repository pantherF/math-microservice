// @generated automatically by Diesel CLI.

diesel::table! {
    calculations (id) {
        id -> Integer,
        operator -> Text,
        first_number -> Double,
        second_number -> Double,
    }
}
