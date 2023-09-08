//! This module contains the non-scalar output values for usage during graphql query/mutation resolution.

// pub mod teacher {
//     //! This module contains all the non-scalar GraphQL types that are mainly related to [Teachers][Teacher].
    
//     use std::fmt::Display;

//     use crate::utils::structs::TeacherRow;

//     use super::super::scalars::teacher::*;
//     use super::absence_state::*;

    
// }

// pub mod absence_state {
//     //! This module contains all the non-scalar GraphQL types that are mainly related to 
//     //!  - [GraphQLAbsenceState outputs][GraphQLAbsenceState].
//     //!  - [AbsenceState enums][AbsenceState].

//     use super::period::Period;

//     /// GraphQLAbsenceState is a struct representing the state of absence of its containing struct. (Currently, [Teacher][super::Teacher].)
//     /// Because sum-type enums are not easily implemented by GraphQLObject, there are invalid yet representable states.
//     /// Because of that, some leniency is allowed.
//     /// - If `is_fully_absent` is true, the teacher is fully absent, full stop.
//     /// - If `is_absent` is true, `absent_periods` SHOULD be `Some` of a non-empty `Vec`
//     ///     - If it is, the teacher is out for those periods.
//     ///     - If it is not, `****`
//     /// - Otherwise, the teacher is fully present for all periods of the day.
//     /// 
//     /// Generally, try to use [AbsenceState] in internal code, and only use this struct when replying to queries.
//     /// 
//     /// TODO: Figure out what to do with `true`, `false`, `None`.
//     #[derive(juniper::GraphQLObject, Debug, Clone)]
//     pub struct GraphQLAbsenceState {
//         /// A flag for whether a teach is absent for an ENTIRE day.
//         /// It is the highest priority, and if it is true, then the following are generally irrelevant
//         is_fully_absent: bool,

//         /// A flag for whether or not a teacher is absent for ANY portion or the day.
//         /// If it is false, check `is_fully_absent`.
//         is_absent: bool,
        
//         /// The Periods that the teacher is absent for, if they are absent at all.
//         absent_periods: Option<Vec<Period>>
//     }

//     /// A struct that represents a [Teacher]'s state of absence.
//     /// 
//     /// This struct confines invalid states to the `Invalid` variant.
//     /// As noted in [GraphQLAbsenceState], it needs to be decided what to do with an `Invalid` Variant.
//     /// At this point, probably just return a contract violation error.
//     pub enum AbsenceState {
//         /// Represents a teacher that is present for the ENTIRE day.
//         Present,
//         /// Represents a teacher that is present for all periods except the ones contained. (The `Vec` should have at least 1 period defined.)
//         PartiallyAbsent(Vec<Period>),
//         /// Represents a teacher that is absent for the ENTIRE day.
//         FullyAbsent(Vec<Period>),
//     }

//     impl TryFrom<GraphQLAbsenceState> for AbsenceState {
//         type Error = GraphQLAbsenceState;
        
//         fn try_from(graphql_object: GraphQLAbsenceState) -> Result<Self, GraphQLAbsenceState> {
//             use AbsenceState::*;
//             match graphql_object {
//                 GraphQLAbsenceState { is_fully_absent: true, absent_periods: Some(periods), .. } => Ok(FullyAbsent(periods)),
//                 GraphQLAbsenceState { is_absent: true, absent_periods: Some(periods), .. } => Ok(PartiallyAbsent(periods)),
//                 _ => Ok(Present),
//             }
//         }
//     }

//     impl From<AbsenceState> for GraphQLAbsenceState {
//         fn from(absence_state: AbsenceState) -> Self {
//             use AbsenceState::*;
//             match absence_state {
//                 Present => GraphQLAbsenceState { is_fully_absent: false, is_absent: false, absent_periods: None },
//                 PartiallyAbsent(periods) => GraphQLAbsenceState { is_fully_absent: false, is_absent: true, absent_periods: Some(periods) },
//                 FullyAbsent(periods) => GraphQLAbsenceState { is_fully_absent: true, is_absent: true, absent_periods: Some(periods) },
//             }
//         }
//     }
// }

// pub mod period {
//     //! This module contains all the non-scalar GraphQL types that are mainly related to [Periods][Period].
    
//     use std::borrow::Cow;
//     use const_format::formatcp;
//     use tokio_postgres::Row;

//     use super::super::scalars::period::*;

//     /// A period as represented externally to the client and internally to the server.
//     /// The ID and Name are essentially graphql-compatible newtypes.
//     #[derive(juniper::GraphQLObject, Debug, Clone)]
//     pub struct Period {
//         /// The id of the period. A UUID wrapper that is (de)serializable for juniper.
//         pub id: PeriodId,
//         /// The name of the period. A String wrapper.
//         pub name: PeriodName,
//     }

//     impl TryFrom<Row> for Period {
//         type Error = Cow<'static, str>;
//         fn try_from(row: Row) -> Result<Self, Self::Error> {
//             /// FIXME: Centralize this constant.
//             const COL_NAMES: [&str; 2] = ["periodid", "periodname"];
    
//             match (row.try_get(COL_NAMES[0]), row.try_get(COL_NAMES[1])) {
//                 (Ok(id), Ok(name)) => Ok(Period {
//                     id: PeriodId::new(&id),
//                     name: PeriodName::new(name), 
//                 }),
//                 (Ok(_), Err(_)) => Err(formatcp!("Row does not contain {:?}", COL_NAMES[1]).into()),
//                 (Err(_), Ok(_)) => Err(formatcp!("Row does not contain {:?}", COL_NAMES[0]).into()),
//                 (Err(_), Err(_)) => Err(formatcp!("Row does not contain {:?}, {:?}", COL_NAMES[0], COL_NAMES[1]).into()),
//             }
    
//         }
//     }
// }