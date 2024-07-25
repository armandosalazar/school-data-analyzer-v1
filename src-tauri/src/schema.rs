// @generated automatically by Diesel CLI.

diesel::table! {
    divisions (id) {
        id -> Nullable<Integer>,
        code -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    grades (id) {
        id -> Nullable<Integer>,
        student_id -> Nullable<Integer>,
        subject_id -> Nullable<Integer>,
        first_grade -> Nullable<Integer>,
        second_grade -> Nullable<Integer>,
        third_grade -> Nullable<Integer>,
        first_faults -> Nullable<Integer>,
        second_faults -> Nullable<Integer>,
        third_faults -> Nullable<Integer>,
        first_weighing -> Nullable<Integer>,
        second_weighing -> Nullable<Integer>,
        third_weighing -> Nullable<Integer>,
    }
}

diesel::table! {
    specialities (id) {
        id -> Nullable<Integer>,
        code -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    students (id) {
        id -> Nullable<Integer>,
        speciality_id -> Nullable<Integer>,
        register -> Nullable<Integer>,
        name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        status -> Nullable<Text>,
        semester -> Nullable<Integer>,
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
        payroll -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

diesel::joinable!(grades -> students (student_id));
diesel::joinable!(grades -> subjects (subject_id));
diesel::joinable!(students -> specialities (speciality_id));
diesel::joinable!(subjects -> divisions (division_id));
diesel::joinable!(subjects -> teachers (teacher_id));

diesel::allow_tables_to_appear_in_same_query!(
    divisions,
    grades,
    specialities,
    students,
    subjects,
    teachers,
);
