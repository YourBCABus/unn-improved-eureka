macro_rules! define_scopes {
    ($($public:vis $scopes:ident),+ $(,)?) => {
        pub const SCOPES: &[&str] = &[$(stringify!($scopes)),+];

        paste::paste! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            
            pub enum Scope {
                $(
                    #[doc = "Create a new `" $scopes "` object."]
                    [<$scopes:upper:camel>]
                ),+
            }
        }

        paste::paste! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Scopes {
                $(pub [<$scopes:snake>]: bool),+
            }
        }

        impl Default for Scopes {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Scopes {
            pub const fn new() -> Self {
                paste::paste! {
                    Self {
                        $([<$scopes:snake>]: define_scopes!(@inner $public)),+
                    }
                }
            }

            pub fn try_from_str(s: &str) -> Option<Self> {
                let mut scopes = Self::new();
                for scope in s.split(' ') {
                    paste::paste! {
                        match scope {
                            $(stringify!($scopes) => scopes.[<$scopes:snake>] = true,)+
                            _ => return None,
                        }
                    }
                }
                Some(scopes)
            }

            pub fn to_string(&self) -> String {
                let mut s = String::new();
                $(if self.$scopes { s.push_str(stringify!($scopes)); s.push(' '); })+
                s.pop();
                s
            }

            pub fn has(&self, scope: &Scope) -> bool {
                let base = paste::paste! {
                    match scope {
                        $(Scope::[<$scopes:upper:camel>] => self.[<$scopes:snake>],)+
                    }
                };

                if self.admin {
                    true
                } else {
                    base
                }
            }

            pub fn all() -> Self {
                Self::new() | !Self::new()
            }

            pub fn none() -> Self {
                Self::new() & !Self::new()
            }
        }

        impl std::ops::BitOr for Scopes {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                let mut scopes = Self::new();
                paste::paste! {
                    $(scopes.[<$scopes:snake>] = self.[<$scopes:snake>] || rhs.[<$scopes:snake>];)+
                }
                scopes
            }
        }

        impl std::ops::BitAnd for Scopes {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                let mut scopes = Self::new();
                paste::paste! {
                    $(scopes.[<$scopes:snake>] = self.[<$scopes:snake>] && rhs.[<$scopes:snake>];)+
                }
                scopes
            }
        }

        impl std::ops::Not for Scopes {
            type Output = Self;

            fn not(self) -> Self {
                let mut scopes = Self::new();
                paste::paste! {
                    $(scopes.[<$scopes:snake>] = !self.[<$scopes:snake>];)+
                }
                scopes
            }
        }
    };
    (@inner pub) => {
        true
    };
    (@inner $discard:vis) => {
        false
    };
}

define_scopes!(
    // Read scopes
    pub read_teacher,
    pub read_teacher_name,
    pub read_teacher_pronouns,
    pub read_teacher_absence,

    pub read_period,


    // Write scopes
    write_teacher_name,
    write_teacher_pronouns,
    write_teacher_absence,
    
    write_period_name,
    write_period_temp_time,
    write_period_time,


    // Create/delete scopes
    create_period,
    create_teacher,

    delete_period,
    delete_teacher,


    // Admin scopes
    write_config,
    experimental,
    admin,
);
