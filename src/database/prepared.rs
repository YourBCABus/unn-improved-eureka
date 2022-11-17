pub mod modifying {
    use crate::preludes::macros::define_shared_query_name;

    define_shared_query_name!(pub add_teacher_query: "INSERT INTO teachers (TeacherName) VALUES($1);");

    define_shared_query_name!(pub delete_teacher_query: "DELETE FROM teachers WHERE TeacherId = $1;");
}

pub mod read {
    use crate::preludes::macros::define_shared_query_name;

    define_shared_query_name!(pub all_teachers: "SELECT * FROM Teachers");

    define_shared_query_name!(pub get_teacher_by_name_query: "SELECT * FROM Teachers WHERE TeacherName = $1");
    define_shared_query_name!(pub get_teacher_by_id_query: "SELECT * FROM Teachers WHERE TeacherId = $1");

    define_shared_query_name!(pub teacher_id_by_name_query: "SELECT TeacherId FROM Teachers WHERE TeacherName = $1");
}
