#![allow(non_snake_case)]
#![allow(clippy::missing_docs_in_private_items)]

pub mod Teachers {
    pub mod TeacherId {
        pub type Type = uuid::Uuid;
        pub static COL_IDX: &str = "TeacherId";
        pub fn get_teacher_id(row: tokio_postgres::Row) -> Type {
            row.try_get(COL_IDX).unwrap_or_default()
        }
        pub fn try_get_teacher_id(row: tokio_postgres::Row) -> Option<Type> {
            row.try_get(COL_IDX).ok()
        }
    }

    pub mod TeacherName {
        pub type Type = String;
        pub static COL_IDX: &str = "TeacherName";
        pub fn get_teacher_id(row: tokio_postgres::Row) -> Type {
            row.try_get(COL_IDX).unwrap_or_default()
        }
        pub fn try_get_teacher_id(row: tokio_postgres::Row) -> Option<Type> {
            row.try_get(COL_IDX).ok()
        }
    }

    pub mod TeacherPresence {
        use crate::utils::macros::make_sql_enum;
        pub type Type = TeacherPresence;


        make_sql_enum!{
            pub TeacherPresence
            FullPresent => "full_present"
            PartAbsent => "part_absent"
            FullAbsent => "full_absent"
        }
    }
}

