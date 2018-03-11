/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashSet;

pub struct CTypeResolver {
        structs: HashSet<String>,
        enums: HashSet<String>,
        unions: HashSet<String>,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum CType {
    Struct,
    Enum,
    Union,
}

impl CType {
    pub fn to_str(&self) -> &'static str {
        match self {
            &CType::Struct => "struct",
            &CType::Enum => "enum",
            &CType::Union => "union",
        }
    }
}

impl CTypeResolver {
    pub fn new() -> CTypeResolver {
        CTypeResolver {
            structs: HashSet::new(),
            enums: HashSet::new(),
            unions: HashSet::new(),
        }
    }

    pub fn add_enum(&mut self, name: &str) {
        self.enums.insert(name.to_owned());
    }

    pub fn add_struct(&mut self, name: &str) {
        self.structs.insert(name.to_owned());
    }

    pub fn add_union(&mut self, name: &str) {
        self.unions.insert(name.to_owned());
    }

    pub fn type_for(&self, name: &str) -> Option<CType> {
        if self.structs.contains(name) {
            Some(CType::Struct)
        } else if self.enums.contains(name) {
            Some(CType::Enum)
        } else if self.unions.contains(name) {
            Some(CType::Union)
        } else {
            None
        }
    }
}