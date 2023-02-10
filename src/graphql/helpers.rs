pub mod teachers {
    use uuid::Uuid;

    use crate::utils::list_to_value;
    use crate::database::prelude::*;

    use crate::{
        preludes::graphql::*,
        graphql_types::{
            teachers::*,
            periods::*,
            objects::absence_state::*,
        },
    };

    use crate::macros::{
        handle_prepared,
        make_unit_enum_error,
        make_static_enum_error,
    };

    use crate::utils::structs::TeacherRow;

    make_unit_enum_error! {
        /// Database execution errors
        pub DbExecError
            GetPeriods => "get_teacher_periods"
    }
    
    make_static_enum_error! {
        
        /// This struct contains all the possible error types that can occur when executing the get_teacher query.
        /// 4 of the types are client errors (C), 4 are server errors (S).
        pub PopulateTeacherAbsenceError;
            /// S - A prepared query (&Statement) failed to load due to some error. Contains the names of the queries.
            PreparedQueryError(Vec<&'static str>)
                => "1 or more prepared queries failed.",
                    "prepared_fail" ==> |failed_list| {
                        "failed": list_to_value(failed_list),
                    };
            /// S - Something went wrong while running a specific SQL query.
            ExecError(DbExecError)
                => "Database error",
                    "db_failed" ==> |error_type| {
                        "part_failed": error_type.error_str(),
                    };
            /// S - Something else went wrong with the database.
            OtherDbError(String)
                => "Unknown database error",
                    "db_failed" ==> |reason| {
                        "reason": reason,
                    };
            // /// S - Catch-all for other things.
            // Other(String)
            //     => "Unknown server error",
            //         "server_failed" ==> |reason| {
            //             "reason": reason,
            //         };
    }

    pub async fn populate_absence(teacher_row: TeacherRow, db_client: &Client) -> Result<Teacher, PopulateTeacherAbsenceError> {
        if let TeacherPresence::FullPresent = teacher_row.presence {
            
            Ok(Teacher::from_row_and_state(teacher_row, AbsenceState::Present))
        } else {

            let gtp = read::get_periods_from_teacher_query(db_client).await;

            let gtp = handle_prepared!(
                gtp;
                PopulateTeacherAbsenceError::PreparedQueryError
            )?;

            let periods = get_teacher_periods(&teacher_row.id.uuid(), db_client, gtp).await?;

            println!("{:?}", periods);

            match teacher_row.presence {
                TeacherPresence::FullAbsent => Ok(Teacher::from_row_and_state(
                    teacher_row,
                    AbsenceState::FullyAbsent(periods),
                )),
                TeacherPresence::PartAbsent => Ok(Teacher::from_row_and_state(
                    teacher_row,
                    AbsenceState::PartiallyAbsent(periods),
                )),
                TeacherPresence::FullPresent => unreachable!(),
            }
        }
    }

    async fn get_teacher_periods(teacher_id: &Uuid, transaction: &Client, gtp: &Statement) -> Result<Vec<Period>, PopulateTeacherAbsenceError> {
        let periods = transaction
            .query(gtp, &[&teacher_id])
            .await
            .map_err(|_| PopulateTeacherAbsenceError::ExecError(DbExecError::GetPeriods))?;
        
        periods.into_iter()
            .map(|period_row| Period
                ::try_from(period_row)
                .map_err(|err| PopulateTeacherAbsenceError::OtherDbError(err))
            )
            .collect()
    }

}