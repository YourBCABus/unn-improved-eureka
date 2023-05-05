//! This crate contains [modifying] and [read] for the two flavors of SQL query.
//! See their documentation for more information.

pub mod modifying {
    //! This module defines solely SQL queries that intend to modify the state of the database and/or the server.
    //! If you want to add one, MAKE SURE THAT IT MODIFIES in at least one case.
    use crate::preludes::macros::define_shared_query_name;

    // Teacher-specific modifying sql queries:
    define_shared_query_name!(pub add_teacher_query: "
        INSERT INTO teachers
            (firstname, lastname, isAbsent, fullyAbsent, honorific, pronouns)
            VALUES($1, $2, false, false, $3, $4);
    ");
    define_shared_query_name!(pub update_teacher_metadata_query: "
        UPDATE teachers
        SET
            firstname = $2,
            lastname = $3,
            honorific = $4,
            pronouns = $5
        WHERE teacherId = $1;
    ");
    define_shared_query_name!(pub update_teacher_absence_metadata_query: "
        UPDATE teachers
        SET
            isAbsent = $2,
            fullyAbsent = $3
        WHERE teacherId = $1;
    ");
    // define_shared_query_name!(pub update_teacher_query: "
    //     UPDATE teachers
    //     SET
    //         teacherName = $2,
    //         isAbsent = $3,
    //         fullyAbsent = $4,
    //         honorific = $5,
    //         pronouns = $6
    //     WHERE teacherId = $1;
    // ");
    define_shared_query_name!(pub delete_teacher_query: "DELETE FROM teachers WHERE TeacherId = $1;");
    
    // Period-specific modifying sql queries:
    define_shared_query_name!(pub add_period_query: "
        INSERT INTO Periods
            (PeriodName, UtcStartDefault, UtcEndDefault)
            VALUES ($1, $2, $3);
    ");
    define_shared_query_name!(pub update_period_query: "
        UPDATE periods
        SET
            PeriodName = $2,
            UtcStartDefault = $3, UtcEndDefault = $4,
            UtcStartCurrent = $5, UtcEndCurrent = $6
        WHERE PeriodId = $1;");
    define_shared_query_name!(pub delete_period_query: "DELETE FROM periods WHERE PeriodId = $1;");
    define_shared_query_name!(pub clear_temp_times: "UPDATE periods SET UtcStartCurrent = NULL, UtcEndCurrent = NULL");

    
    // Teacher-Period relation modifying sql queries:
    define_shared_query_name!(pub add_teacher_periods: "INSERT INTO Teachers_Periods_Absence_XRef VALUES ($1, $2);");
    define_shared_query_name!(pub clear_periods_for_teacher_query: "DELETE FROM Teachers_Periods_Absence_XRef WHERE TeacherId = $1;");
    define_shared_query_name!(pub clear_teachers_for_period_query: "DELETE FROM Teachers_Periods_Absence_XRef WHERE PeriodId = $1;");
    define_shared_query_name!(pub clear_absences: "DELETE FROM Teachers_Periods_Absence_XRef");
    define_shared_query_name!(pub clear_absence_metadata: "UPDATE teachers SET isAbsent = false, fullyAbsent = false;");
    
}

pub mod read {
    //! This module defines solely SQL queries that do not and can not modify the state of the database and/or the server.
    //! If you want to add one, MAKE SURE THAT IT DOES NOT MODIFY in ANY case.

    use crate::preludes::macros::define_shared_query_name;

    define_shared_query_name!(pub all_teachers_query: "SELECT * FROM Teachers");
    define_shared_query_name!(pub get_teacher_by_name_query: "SELECT * FROM Teachers WHERE firstname = $1 AND lastname = $2");
    define_shared_query_name!(pub get_teacher_by_id_query: "SELECT * FROM Teachers WHERE TeacherId = $1");
    define_shared_query_name!(pub teacher_id_by_name_query: "SELECT TeacherId FROM Teachers WHERE WHERE firstname = $1 AND lastname = $2");
    
    
    define_shared_query_name!(pub all_periods_query: "SELECT * FROM Periods");
    define_shared_query_name!(pub get_period_by_id_query: "SELECT * FROM Periods WHERE PeriodId = $1");
    define_shared_query_name!(pub get_period_by_name_query: "SELECT * FROM Periods WHERE PeriodName = $1");
    

    define_shared_query_name!(pub get_teachers_from_period_query: "
        SELECT Teachers.* FROM Teachers_Periods_Absence_XRef
        INNER JOIN Teachers ON Teachers_Periods_Absence_XRef.TeacherId = Teachers.TeacherId
        WHERE Teachers_Periods_Absence_XRef.PeriodId = $1
    ");
    define_shared_query_name!(pub get_periods_from_teacher_query: "
        SELECT Periods.* FROM Teachers_Periods_Absence_XRef
        INNER JOIN Periods ON Teachers_Periods_Absence_XRef.PeriodId = Periods.PeriodId
        WHERE Teachers_Periods_Absence_XRef.TeacherId = $1
    ");
}
