//! This module contains most of the structs for sending and processing during graphql query resolution.
//! 

pub use teacher::*;
pub use absence_state::*;
pub use period::*;

mod teacher {
    //! This module is just an organizational construct to contain everything only used by the [Teacher] struct.
    
    use std::fmt::Display;
    
    use tokio_postgres::Row;
    
    use crate::utils::macros::{make_id_wrapper, make_name_wrapper};

    use super::absence_state::*;

    /// This struct represents a Teacher, both as treated by the graphql client and this server.
    /// More info is included in the inputs
    #[derive(juniper::GraphQLObject, Debug, Clone)]
    pub struct Teacher {
        /// The id of the teacher. This is essentially a wrapper for a UUID, but is (de)serializable for juniper.
        pub id: TeacherId,
        /// The name of the teacher. A String wrapper.
        pub name: TeacherName,
        /// The absence of the teacher.
        pub absence_state: GraphQLAbsenceState,
    }

    impl TryFrom<Row> for Teacher {
        type Error = String;
        fn try_from(row: Row) -> Result<Self, Self::Error> {
            /// FIXME: Centralize this constant.
            const COL_NAMES: [&str; 2] = ["teacherid", "teachername"];
    
            match (row.try_get(COL_NAMES[0]), row.try_get(COL_NAMES[1])) {
                (Ok(id), Ok(name)) => Ok(Teacher {
                    id: TeacherId::new(&id),
                    name: TeacherName::new(name), 
                    absence_state: AbsenceState::Present.into(),
                }),
                (Ok(_), Err(_)) => Err(format!("Row does not contain {:?}", [COL_NAMES[1]])),
                (Err(_), Ok(_)) => Err(format!("Row does not contain {:?}", [COL_NAMES[0]])),
                (Err(_), Err(_)) => Err(format!("Row does not contain {:?}", [COL_NAMES[0], COL_NAMES[1]])),
            }
    
        }
    }
    
    impl Display for Teacher {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Teacher<{}> ({})", self.name.name_str(), self.id.id_str())
        }
    }

    make_id_wrapper!{
        /// The TeacherId struct's only purpose is to verify that it can only be made from a uuid,
        /// and should only represent the ID of a teacher to prevent a mix-and-match of ID types.
        pub TeacherId
    }
    impl Display for TeacherId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "id<{}>", self.id_str())
        }
    }

    make_name_wrapper!{
        /// The PeriodName's only job is to prevent a mix-and-match of different names.
        pub TeacherName
    }
}



mod absence_state {
    //! This module is just an organizational construct to contain everything only used by the [GraphQLAbsenceState] and [AbsenceState] structs.


    use super::period::Period;

    /// GraphQLAbsenceState is a struct representing the state of absence of its containing struct. (Currently, [Teacher][super::Teacher].)
    /// Because sum-type enums are not easily implemented by GraphQLObject, there are invalid yet representable states.
    /// Because of that, some leniency is allowed.
    /// - If `is_fully_absent` is true, the teacher is fully absent, full stop.
    /// - If `is_absent` is true, `absent_periods` SHOULD be `Some` of a non-empty `Vec`
    ///     - If it is, the teacher is out for those periods.
    ///     - If it is not, `****`
    /// - Otherwise, the teacher is fully present for all periods of the day.
    /// 
    /// Generally, try to use [AbsenceState] in internal code, and only use this struct when replying to queries.
    /// 
    /// TODO: Figure out what to do with `true`, `false`, `None`.
    #[derive(juniper::GraphQLObject, Debug, Clone)]
    pub struct GraphQLAbsenceState {
        /// A flag for whether a teach is absent for an ENTIRE day.
        /// It is the highest priority, and if it is true, then the following are generally irrelevant
        is_fully_absent: bool,

        /// A flag for whether or not a teacher is absent for ANY portion or the day.
        /// If it is false, check `is_fully_absent`.
        is_absent: bool,
        
        /// The Periods that the teacher is absent for, if they are absent at all.
        absent_periods: Option<Vec<Period>>
    }

    /// A struct that represents a [Teacher]'s state of absence.
    /// 
    /// This struct confines invalid states to the `Invalid` variant.
    /// As noted in [GraphQLAbsenceState], it needs to be decided what to do with an `Invalid` Variant.
    /// At this point, probably just return a contract violation error.
    pub enum AbsenceState {
        /// Represents a teacher that is present for the ENTIRE day.
        Present,
        /// Represents a teacher that is present for all periods except the ones contained. (The `Vec` should have at least 1 period defined.)
        PartiallyAbsent(Vec<Period>),
        /// Represents a teacher that is absent for the ENTIRE day.
        FullyAbsent,
    }

    impl TryFrom<GraphQLAbsenceState> for AbsenceState {
        type Error = GraphQLAbsenceState;
        
        fn try_from(graphql_object: GraphQLAbsenceState) -> Result<Self, GraphQLAbsenceState> {
            use AbsenceState::*;
            match graphql_object {
                GraphQLAbsenceState { is_fully_absent: true, .. } => Ok(FullyAbsent),
                GraphQLAbsenceState { is_absent: true, absent_periods: Some(periods), .. } => if !periods.is_empty() {
                    Ok(PartiallyAbsent(periods))
                } else {
                    Err(GraphQLAbsenceState { absent_periods: Some(periods), ..graphql_object })
                },
                _ => Ok(Present),
            }
        }
    }

    impl From<AbsenceState> for GraphQLAbsenceState {
        fn from(absence_state: AbsenceState) -> Self {
            use AbsenceState::*;
            match absence_state {
                Present => GraphQLAbsenceState { is_fully_absent: false, is_absent: false, absent_periods: None },
                PartiallyAbsent(periods) => GraphQLAbsenceState { is_fully_absent: false, is_absent: true, absent_periods: Some(periods) },
                FullyAbsent => GraphQLAbsenceState { is_fully_absent: true, is_absent: true, absent_periods: None },
            }
        }
    }
}

mod period {
    //! This module is just an organizational construct to contain everything only used by the [Period] struct.
    //! 
    use tokio_postgres::Row;

    use crate::utils::macros::{make_id_wrapper, make_name_wrapper};

    make_id_wrapper!{
        /// The PeriodId struct's only purpose is to verify that it can only be made from a uuid,
        /// and should only represent the ID of a period to prevent a mix-and-match of ID types.
        pub PeriodId
    }

    make_name_wrapper!{
        /// The PeriodName's only job is to prevent a mix-and-match of different names.
        pub PeriodName
    }

    /// A period as represented externally to the client and internally to the server.
    /// The ID and Name are essentially graphql-compatible newtypes.
    #[derive(juniper::GraphQLObject, Debug, Clone)]
    pub struct Period {
        /// The id of the period. A UUID wrapper that is (de)serializable for juniper.
        pub id: PeriodId,
        /// The name of the period. A String wrapper.
        pub name: PeriodName,
    }

    impl TryFrom<Row> for Period {
        type Error = String;
        fn try_from(row: Row) -> Result<Self, Self::Error> {
            /// FIXME: Centralize this constant.
            const COL_NAMES: [&str; 2] = ["periodid", "periodname"];
    
            match (row.try_get(COL_NAMES[0]), row.try_get(COL_NAMES[1])) {
                (Ok(id), Ok(name)) => Ok(Period {
                    id: PeriodId::new(&id),
                    name: PeriodName::new(name), 
                }),
                (Ok(_), Err(_)) => Err(format!("Row does not contain {:?}", [COL_NAMES[1]])),
                (Err(_), Ok(_)) => Err(format!("Row does not contain {:?}", [COL_NAMES[0]])),
                (Err(_), Err(_)) => Err(format!("Row does not contain {:?}", [COL_NAMES[0], COL_NAMES[1]])),
            }
    
        }
    }
}



