// DO NOT EDIT
// This file was generated by Stone

#![allow(
    unknown_lints,  // keep rustc from complaining about clippy lints
    too_many_arguments,
    large_enum_variant,
    doc_markdown,
)]

pub type Date = String /*Timestamp*/;
pub type DisplayName = String;
pub type DisplayNameLegacy = String;
pub type DropboxTimestamp = String /*Timestamp*/;
pub type EmailAddress = String;
pub type LanguageCode = String;
pub type NamePart = String;
pub type NamespaceId = String;
pub type OptionalNamePart = String;
pub type PathRootId = NamespaceId;
pub type SessionId = String;
pub type SharedFolderId = NamespaceId;

#[derive(Debug)]
pub struct InvalidPathRootError {
    /// The latest path root id for user's team if the user is still in a team.
    pub path_root: Option<PathRootId>,
}

impl Default for InvalidPathRootError {
    fn default() -> Self {
        InvalidPathRootError {
            path_root: None,
        }
    }
}

const INVALID_PATH_ROOT_ERROR_FIELDS: &'static [&'static str] = &["path_root"];
impl InvalidPathRootError {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
    ) -> Result<InvalidPathRootError, V::Error> {
        use serde::de;
        let mut field_path_root = None;
        while let Some(key) = map.next_key()? {
            match key {
                "path_root" => {
                    if field_path_root.is_some() {
                        return Err(de::Error::duplicate_field("path_root"));
                    }
                    field_path_root = Some(map.next_value()?);
                }
                _ => return Err(de::Error::unknown_field(key, INVALID_PATH_ROOT_ERROR_FIELDS))
            }
        }
        Ok(InvalidPathRootError {
            path_root: field_path_root,
        })
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("path_root", &self.path_root)
    }
}

impl<'de> ::serde::de::Deserialize<'de> for InvalidPathRootError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = InvalidPathRootError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a InvalidPathRootError struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                InvalidPathRootError::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("InvalidPathRootError", INVALID_PATH_ROOT_ERROR_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for InvalidPathRootError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("InvalidPathRootError", 1)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

#[derive(Debug)]
pub enum PathRoot {
    /// Paths are relative to the authenticating user's home directory, whether or not that user
    /// belongs to a team.
    Home,
    /// Paths are relative to the authenticating team member's home directory. (This results in
    /// :field:`PathRootError.invalid` if the user does not belong to a team.).
    MemberHome,
    /// Paths are relative to the given team directory. (This results in
    /// :field:`PathRootError.invalid` if the user is not a member of the team associated with that
    /// path root id.).
    Team(PathRootId),
    /// Paths are relative to the user's home directory. (This results in
    /// :field:`PathRootError.invalid` if the belongs to a team.).
    UserHome,
    /// Paths are relative to given namespace id (This results in
    /// :field:`PathRootError.no_permission` if you don't have access to this namespace.).
    NamespaceId(PathRootId),
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for PathRoot {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = PathRoot;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a PathRoot structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "home" => Ok(PathRoot::Home),
                    "member_home" => Ok(PathRoot::MemberHome),
                    "team" => {
                        match map.next_key()? {
                            Some("team") => Ok(PathRoot::Team(map.next_value()?)),
                            None => Err(de::Error::missing_field("team")),
                            _ => Err(de::Error::unknown_field(tag, VARIANTS))
                        }
                    }
                    "user_home" => Ok(PathRoot::UserHome),
                    "namespace_id" => {
                        match map.next_key()? {
                            Some("namespace_id") => Ok(PathRoot::NamespaceId(map.next_value()?)),
                            None => Err(de::Error::missing_field("namespace_id")),
                            _ => Err(de::Error::unknown_field(tag, VARIANTS))
                        }
                    }
                    _ => Ok(PathRoot::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["home",
                                                    "member_home",
                                                    "team",
                                                    "user_home",
                                                    "namespace_id",
                                                    "other"];
        deserializer.deserialize_struct("PathRoot", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for PathRoot {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            PathRoot::Home => {
                // unit
                let mut s = serializer.serialize_struct("PathRoot", 1)?;
                s.serialize_field(".tag", "home")?;
                s.end()
            }
            PathRoot::MemberHome => {
                // unit
                let mut s = serializer.serialize_struct("PathRoot", 1)?;
                s.serialize_field(".tag", "member_home")?;
                s.end()
            }
            PathRoot::Team(ref x) => {
                // primitive
                let mut s = serializer.serialize_struct("{}", 2)?;
                s.serialize_field(".tag", "team")?;
                s.serialize_field("team", x)?;
                s.end()
            }
            PathRoot::UserHome => {
                // unit
                let mut s = serializer.serialize_struct("PathRoot", 1)?;
                s.serialize_field(".tag", "user_home")?;
                s.end()
            }
            PathRoot::NamespaceId(ref x) => {
                // primitive
                let mut s = serializer.serialize_struct("{}", 2)?;
                s.serialize_field(".tag", "namespace_id")?;
                s.serialize_field("namespace_id", x)?;
                s.end()
            }
            PathRoot::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

#[derive(Debug)]
pub enum PathRootError {
    /// The path root id value in Dropbox-API-Path-Root header is no longer valid.
    Invalid(InvalidPathRootError),
    /// You don't have permission to access the path root id in Dropbox-API-Path-Root  header.
    NoPermission,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for PathRootError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = PathRootError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a PathRootError structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "invalid" => Ok(PathRootError::Invalid(InvalidPathRootError::internal_deserialize(map)?)),
                    "no_permission" => Ok(PathRootError::NoPermission),
                    _ => Ok(PathRootError::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["invalid",
                                                    "no_permission",
                                                    "other"];
        deserializer.deserialize_struct("PathRootError", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for PathRootError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            PathRootError::Invalid(ref x) => {
                // struct
                let mut s = serializer.serialize_struct("PathRootError", 2)?;
                s.serialize_field(".tag", "invalid")?;
                x.internal_serialize::<S>(&mut s)?;
                s.end()
            }
            PathRootError::NoPermission => {
                // unit
                let mut s = serializer.serialize_struct("PathRootError", 1)?;
                s.serialize_field(".tag", "no_permission")?;
                s.end()
            }
            PathRootError::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for PathRootError {
    fn description(&self) -> &str {
        "PathRootError"
    }
}

impl ::std::fmt::Display for PathRootError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

