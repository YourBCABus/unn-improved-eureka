

pub mod teacher {
    use std::fmt::Display;
    use tokio_postgres::Row;

    use crate::utils::structs::TeacherRow;

    use super::super::scalars::teacher::*;
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

    impl Teacher {
        pub fn from_row_and_state(row: TeacherRow, state: AbsenceState) -> Self {
            Self {
                id: row.id,
                name: row.name,
                absence_state: state.into(),
            }
        }
    }

    // impl TryFrom<Row> for Teacher {
    //     type Error = String;
    //     fn try_from(row: Row) -> Result<Self, Self::Error> {
    //         /// FIXME: Centralize this constant.
    //         const COL_NAMES: [&str; 4] = ["teacherid", "teachername", "isabsent", "fullyabsent"];
    
    //         match (row.try_get(COL_NAMES[0]), row.try_get(COL_NAMES[1])) {
    //             (Ok(id), Ok(name)) => Ok(Teacher {
    //                 id: TeacherId::new(&id),
    //                 name: TeacherName::new(name), 
    //                 absence_state: match (row.try_get(COL_NAMES[2]), row.try_get(COL_NAMES[3])) {
    //                     (Ok(_), Ok(true)) => AbsenceState::FullyAbsent(Vec::new()),
    //                     (Ok(true), Ok(false)) => AbsenceState::PartiallyAbsent(Vec::new()),
    //                     (Ok(false), Ok(false)) => AbsenceState::Present,
    //                     (a, b) => Err(format!("Row does not contain valid absence state: {:?}, {:?}.", a, b))?,
    //                 }.into(),
    //             }),
    //             (Ok(_), Err(_)) => Err(format!("Row does not contain {:?}", [COL_NAMES[1]])),
    //             (Err(_), Ok(_)) => Err(format!("Row does not contain {:?}", [COL_NAMES[0]])),
    //             (Err(_), Err(_)) => Err(format!("Row does not contain {:?}", [COL_NAMES[0], COL_NAMES[1]])),
    //         }
    
    //     }
    // }
    
    impl Display for Teacher {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Teacher<{}> ({})", self.name.name_str(), self.id.id_str())
        }
    }

    #[derive(juniper::GraphQLObject, Debug, Clone)]
    pub struct TeacherMetadata {
        /// The id of the teacher. This is essentially a wrapper for a UUID, but is (de)serializable for juniper.
        pub id: TeacherId,
        /// The name of the teacher. A String wrapper.
        pub name: TeacherName,
        /// The stripped absence state of the teacher.
        pub absence_state: &'static str,
    }

    impl From<TeacherRow> for TeacherMetadata {
        fn from(row: TeacherRow) -> Self {
            Self {
                id: row.id,
                name: row.name,
                absence_state: row.presence.to_sql_type(),
            }
        }
    }
}

pub mod absence_state {
    //! This module is just an organizational construct to contain everything only used by the [GraphQLAbsenceState] and [AbsenceState] structs.

    use std::fmt::Display;
    use tokio_postgres::Row;

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
        FullyAbsent(Vec<Period>),
    }

    impl TryFrom<GraphQLAbsenceState> for AbsenceState {
        type Error = GraphQLAbsenceState;
        
        fn try_from(graphql_object: GraphQLAbsenceState) -> Result<Self, GraphQLAbsenceState> {
            use AbsenceState::*;
            match graphql_object {
                GraphQLAbsenceState { is_fully_absent: true, absent_periods: Some(periods), .. } => Ok(FullyAbsent(periods)),
                GraphQLAbsenceState { is_absent: true, absent_periods: Some(periods), .. } => Ok(PartiallyAbsent(periods)),
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
                FullyAbsent(periods) => GraphQLAbsenceState { is_fully_absent: true, is_absent: true, absent_periods: Some(periods) },
            }
        }
    }
}

pub mod period {
    use std::fmt::Display;
    use tokio_postgres::Row;

    use super::super::scalars::period::*;

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