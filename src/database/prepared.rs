//! This crate contains [modifying] and [read] for the two flavors of SQL query.
//! See their documentation for more information.

pub mod modifying {
    //! This module defines solely SQL queries that intend to modify the state of the database and/or the server.
    //! If you want to add one, MAKE SURE THAT IT MODIFIES in at least one case.
    use crate::preludes::macros::define_shared_query_name;

    define_shared_query_name!(pub add_teacher_query: "INSERT INTO teachers (TeacherName) VALUES($1);");

    define_shared_query_name!(pub update_teacher_query: "UPDATE teachers SET TeacherName = $2 WHERE TeacherId = $1;");

    define_shared_query_name!(pub delete_teacher_query: "DELETE FROM teachers WHERE TeacherId = $1;");
}

pub mod read {
    //! This module defines solely SQL queries that do not and can not modify the state of the database and/or the server.
    //! If you want to add one, MAKE SURE THAT IT DOES NOT MODIFY in ANY case.
    
    use crate::preludes::macros::define_shared_query_name;

    define_shared_query_name!(pub all_teachers: "SELECT * FROM Teachers");

    define_shared_query_name!(pub get_teacher_by_name_query: "SELECT * FROM Teachers WHERE TeacherName = $1");
    define_shared_query_name!(pub get_teacher_by_id_query: "SELECT * FROM Teachers WHERE TeacherId = $1");

    define_shared_query_name!(pub teacher_id_by_name_query: "SELECT TeacherId FROM Teachers WHERE TeacherName = $1");
}
