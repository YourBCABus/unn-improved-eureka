//! This module contains the scalar values for usage during graphql query/mutation resolution.


pub mod teacher {
    //! This module is just an organizational construct to contain everything primarily used by the [Teacher][super::super::teachers::Teacher] struct.
    
    use std::fmt::Display;
        
    use crate::preludes::graphql::{make_name_wrapper, make_id_wrapper};


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
        /// The TeacherName's only job is to prevent a mix-and-match of different names.
        pub TeacherName
    }
}





pub mod period {
    //! This module is just an organizational construct to contain everything primarily used by the [Period] struct.

    use super::super::{make_id_wrapper, make_name_wrapper};


    make_id_wrapper!{
        /// The PeriodId struct's only purpose is to verify that it can only be made from a uuid,
        /// and should only represent the ID of a period to prevent a mix-and-match of ID types.
        pub PeriodId
    }

    make_name_wrapper!{
        /// The PeriodName's only job is to prevent a mix-and-match of different names.
        pub PeriodName
    }
}



