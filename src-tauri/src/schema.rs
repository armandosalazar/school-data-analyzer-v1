// @generated automatically by Diesel CLI.

diesel::table! {
    divisions (id) {
        id -> Nullable<Integer>,
        code -> Nullable<Integer>,
        academy -> Nullable<Text>,
    }
}

diesel::table! {
    grades (id) {
        id -> Nullable<Integer>,
        student_id -> Nullable<Integer>,
        subject_id -> Nullable<Integer>,
        floults -> Nullable<Integer>,
        value -> Nullable<Float>,
        weighing -> Nullable<Integer>,
        partial -> Nullable<Integer>,
    }
}

diesel::table! {
    specialties (id) {
        id -> Nullable<Integer>,
        code -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    students (id) {
        id -> Nullable<Integer>,
        specialty_id -> Nullable<Integer>,
        register -> Nullable<Integer>,
        name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        status -> Nullable<Text>,
        semester -> Nullable<Text>,
        group -> Nullable<Text>,
        turn -> Nullable<Text>,
        level -> Nullable<Text>,
    }
}

diesel::table! {
    subjects (id) {
        id -> Nullable<Integer>,
        teacher_id -> Nullable<Integer>,
        division_id -> Nullable<Integer>,
        code -> Nullable<Text>,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    teachers (id) {
        id -> Nullable<Integer>,
        payfoll -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

diesel::joinable!(grades -> students (student_id));
diesel::joinable!(grades -> subjects (subject_id));
diesel::joinable!(students -> specialties (specialty_id));
diesel::joinable!(subjects -> divisions (division_id));
diesel::joinable!(subjects -> teachers (teacher_id));

diesel::allow_tables_to_appear_in_same_query!(
    divisions,
    grades,
    specialties,
    students,
    subjects,
    teachers,
);
