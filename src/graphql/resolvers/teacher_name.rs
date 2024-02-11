#![allow(unused_braces)]

use async_graphql::{Object, Enum};
use crate::types::TeacherName;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum)]
enum FormatStyle {
    FirstLast,
    FirstMiddleLast,
    HonorificLast,
    HonorificFirstLast,
    HonorificFirstMiddleLast,
}

#[Object]
impl TeacherName {
    async fn honorific(&self) -> &str { self.get_honorific().str() }    
    async fn first(&self) -> &str { self.get_first() }

    // Teacher will almost never even have middle names, so this isn't really
    // relevant
    #[graphql(complexity = 2)]
    async fn middles(&self) -> Vec<&str> { self.visible_middles().collect() }

    async fn last(&self) -> &str { self.get_last() }

    #[graphql(complexity = 2)]
    async fn formatted(
        &self,
        format_style: FormatStyle
    ) -> String {
        use FormatStyle::*;
        
        let first = self.get_first();
        let last = self.get_last();
        let honorific = self.get_honorific();
        let middles: String = self.visible_middles().flat_map(|s| [s, " "]).collect();

        match format_style {
            FirstLast => format!("{first} {last}"),
            FirstMiddleLast => format!("{first} {middles}{last}"),
            HonorificLast => format!("{honorific} {last}"),
            HonorificFirstLast => format!("{honorific} {first} {last}"),
            HonorificFirstMiddleLast => format!("{honorific} {first} {middles}{last}"),
        }
    }

    #[graphql(complexity = 2)]
    async fn full(&self) -> String { self.longest() }

    #[graphql(complexity = 2)]
    async fn first_last(&self) -> String { self.mid_len() }

    #[graphql(complexity = 2)]
    async fn normal(&self) -> String { self.short() }
}
