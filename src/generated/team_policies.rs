// DO NOT EDIT
// This file was generated by Stone

#![allow(
    unknown_lints,  // keep rustc from complaining about clippy lints
    too_many_arguments,
    large_enum_variant,
    doc_markdown,
)]

#[derive(Debug)]
pub enum EmmState {
    /// Emm token is disabled.
    Disabled,
    /// Emm token is optional.
    Optional,
    /// Emm token is required.
    Required,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for EmmState {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = EmmState;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a EmmState structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "disabled" => Ok(EmmState::Disabled),
                    "optional" => Ok(EmmState::Optional),
                    "required" => Ok(EmmState::Required),
                    _ => Ok(EmmState::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["disabled",
                                                    "optional",
                                                    "required",
                                                    "other"];
        deserializer.deserialize_struct("EmmState", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for EmmState {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            EmmState::Disabled => {
                // unit
                let mut s = serializer.serialize_struct("EmmState", 1)?;
                s.serialize_field(".tag", "disabled")?;
                s.end()
            }
            EmmState::Optional => {
                // unit
                let mut s = serializer.serialize_struct("EmmState", 1)?;
                s.serialize_field(".tag", "optional")?;
                s.end()
            }
            EmmState::Required => {
                // unit
                let mut s = serializer.serialize_struct("EmmState", 1)?;
                s.serialize_field(".tag", "required")?;
                s.end()
            }
            EmmState::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

#[derive(Debug)]
pub enum OfficeAddInPolicy {
    /// Office Add-In is disabled.
    Disabled,
    /// Office Add-In is enabled.
    Enabled,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for OfficeAddInPolicy {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = OfficeAddInPolicy;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a OfficeAddInPolicy structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "disabled" => Ok(OfficeAddInPolicy::Disabled),
                    "enabled" => Ok(OfficeAddInPolicy::Enabled),
                    _ => Ok(OfficeAddInPolicy::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["disabled",
                                                    "enabled",
                                                    "other"];
        deserializer.deserialize_struct("OfficeAddInPolicy", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for OfficeAddInPolicy {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            OfficeAddInPolicy::Disabled => {
                // unit
                let mut s = serializer.serialize_struct("OfficeAddInPolicy", 1)?;
                s.serialize_field(".tag", "disabled")?;
                s.end()
            }
            OfficeAddInPolicy::Enabled => {
                // unit
                let mut s = serializer.serialize_struct("OfficeAddInPolicy", 1)?;
                s.serialize_field(".tag", "enabled")?;
                s.end()
            }
            OfficeAddInPolicy::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

#[derive(Debug)]
pub enum PaperDeploymentPolicy {
    /// All team members have access to Paper.
    Full,
    /// Only whitelisted team members can access Paper. To see which user is whitelisted, check
    /// 'is_paper_whitelisted' on 'account/info'.
    Partial,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for PaperDeploymentPolicy {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = PaperDeploymentPolicy;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a PaperDeploymentPolicy structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "full" => Ok(PaperDeploymentPolicy::Full),
                    "partial" => Ok(PaperDeploymentPolicy::Partial),
                    _ => Ok(PaperDeploymentPolicy::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["full",
                                                    "partial",
                                                    "other"];
        deserializer.deserialize_struct("PaperDeploymentPolicy", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for PaperDeploymentPolicy {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            PaperDeploymentPolicy::Full => {
                // unit
                let mut s = serializer.serialize_struct("PaperDeploymentPolicy", 1)?;
                s.serialize_field(".tag", "full")?;
                s.end()
            }
            PaperDeploymentPolicy::Partial => {
                // unit
                let mut s = serializer.serialize_struct("PaperDeploymentPolicy", 1)?;
                s.serialize_field(".tag", "partial")?;
                s.end()
            }
            PaperDeploymentPolicy::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

#[derive(Debug)]
pub enum PaperEnabledPolicy {
    /// Paper is disabled.
    Disabled,
    /// Paper is enabled.
    Enabled,
    /// Unspecified policy.
    Unspecified,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for PaperEnabledPolicy {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = PaperEnabledPolicy;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a PaperEnabledPolicy structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "disabled" => Ok(PaperEnabledPolicy::Disabled),
                    "enabled" => Ok(PaperEnabledPolicy::Enabled),
                    "unspecified" => Ok(PaperEnabledPolicy::Unspecified),
                    _ => Ok(PaperEnabledPolicy::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["disabled",
                                                    "enabled",
                                                    "unspecified",
                                                    "other"];
        deserializer.deserialize_struct("PaperEnabledPolicy", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for PaperEnabledPolicy {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            PaperEnabledPolicy::Disabled => {
                // unit
                let mut s = serializer.serialize_struct("PaperEnabledPolicy", 1)?;
                s.serialize_field(".tag", "disabled")?;
                s.end()
            }
            PaperEnabledPolicy::Enabled => {
                // unit
                let mut s = serializer.serialize_struct("PaperEnabledPolicy", 1)?;
                s.serialize_field(".tag", "enabled")?;
                s.end()
            }
            PaperEnabledPolicy::Unspecified => {
                // unit
                let mut s = serializer.serialize_struct("PaperEnabledPolicy", 1)?;
                s.serialize_field(".tag", "unspecified")?;
                s.end()
            }
            PaperEnabledPolicy::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

#[derive(Debug)]
pub enum PasswordStrengthPolicy {
    /// User passwords will adhere to the minimal password strength policy.
    MinimalRequirements,
    /// User passwords will adhere to the moderate password strength policy.
    ModeratePassword,
    /// User passwords will adhere to the very strong password strength policy.
    StrongPassword,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for PasswordStrengthPolicy {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = PasswordStrengthPolicy;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a PasswordStrengthPolicy structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "minimal_requirements" => Ok(PasswordStrengthPolicy::MinimalRequirements),
                    "moderate_password" => Ok(PasswordStrengthPolicy::ModeratePassword),
                    "strong_password" => Ok(PasswordStrengthPolicy::StrongPassword),
                    _ => Ok(PasswordStrengthPolicy::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["minimal_requirements",
                                                    "moderate_password",
                                                    "strong_password",
                                                    "other"];
        deserializer.deserialize_struct("PasswordStrengthPolicy", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for PasswordStrengthPolicy {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            PasswordStrengthPolicy::MinimalRequirements => {
                // unit
                let mut s = serializer.serialize_struct("PasswordStrengthPolicy", 1)?;
                s.serialize_field(".tag", "minimal_requirements")?;
                s.end()
            }
            PasswordStrengthPolicy::ModeratePassword => {
                // unit
                let mut s = serializer.serialize_struct("PasswordStrengthPolicy", 1)?;
                s.serialize_field(".tag", "moderate_password")?;
                s.end()
            }
            PasswordStrengthPolicy::StrongPassword => {
                // unit
                let mut s = serializer.serialize_struct("PasswordStrengthPolicy", 1)?;
                s.serialize_field(".tag", "strong_password")?;
                s.end()
            }
            PasswordStrengthPolicy::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

#[derive(Debug)]
pub enum RolloutMethod {
    /// Unlink all.
    UnlinkAll,
    /// Unlink devices with the most inactivity.
    UnlinkMostInactive,
    /// Add member to Exceptions.
    AddMemberToExceptions,
}

impl<'de> ::serde::de::Deserialize<'de> for RolloutMethod {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = RolloutMethod;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a RolloutMethod structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "unlink_all" => Ok(RolloutMethod::UnlinkAll),
                    "unlink_most_inactive" => Ok(RolloutMethod::UnlinkMostInactive),
                    "add_member_to_exceptions" => Ok(RolloutMethod::AddMemberToExceptions),
                    _ => Err(de::Error::unknown_variant(tag, VARIANTS))
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["unlink_all",
                                                    "unlink_most_inactive",
                                                    "add_member_to_exceptions"];
        deserializer.deserialize_struct("RolloutMethod", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for RolloutMethod {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            RolloutMethod::UnlinkAll => {
                // unit
                let mut s = serializer.serialize_struct("RolloutMethod", 1)?;
                s.serialize_field(".tag", "unlink_all")?;
                s.end()
            }
            RolloutMethod::UnlinkMostInactive => {
                // unit
                let mut s = serializer.serialize_struct("RolloutMethod", 1)?;
                s.serialize_field(".tag", "unlink_most_inactive")?;
                s.end()
            }
            RolloutMethod::AddMemberToExceptions => {
                // unit
                let mut s = serializer.serialize_struct("RolloutMethod", 1)?;
                s.serialize_field(".tag", "add_member_to_exceptions")?;
                s.end()
            }
        }
    }
}

/// Policy governing which shared folders a team member can join.
#[derive(Debug)]
pub enum SharedFolderJoinPolicy {
    /// Team members can only join folders shared by teammates.
    FromTeamOnly,
    /// Team members can join any shared folder, including those shared by users outside the team.
    FromAnyone,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for SharedFolderJoinPolicy {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = SharedFolderJoinPolicy;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a SharedFolderJoinPolicy structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "from_team_only" => Ok(SharedFolderJoinPolicy::FromTeamOnly),
                    "from_anyone" => Ok(SharedFolderJoinPolicy::FromAnyone),
                    _ => Ok(SharedFolderJoinPolicy::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["from_team_only",
                                                    "from_anyone",
                                                    "other"];
        deserializer.deserialize_struct("SharedFolderJoinPolicy", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for SharedFolderJoinPolicy {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            SharedFolderJoinPolicy::FromTeamOnly => {
                // unit
                let mut s = serializer.serialize_struct("SharedFolderJoinPolicy", 1)?;
                s.serialize_field(".tag", "from_team_only")?;
                s.end()
            }
            SharedFolderJoinPolicy::FromAnyone => {
                // unit
                let mut s = serializer.serialize_struct("SharedFolderJoinPolicy", 1)?;
                s.serialize_field(".tag", "from_anyone")?;
                s.end()
            }
            SharedFolderJoinPolicy::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

/// Policy governing who can be a member of a folder shared by a team member.
#[derive(Debug)]
pub enum SharedFolderMemberPolicy {
    /// Only a teammate can be a member of a folder shared by a team member.
    Team,
    /// Anyone can be a member of a folder shared by a team member.
    Anyone,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for SharedFolderMemberPolicy {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = SharedFolderMemberPolicy;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a SharedFolderMemberPolicy structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "team" => Ok(SharedFolderMemberPolicy::Team),
                    "anyone" => Ok(SharedFolderMemberPolicy::Anyone),
                    _ => Ok(SharedFolderMemberPolicy::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["team",
                                                    "anyone",
                                                    "other"];
        deserializer.deserialize_struct("SharedFolderMemberPolicy", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for SharedFolderMemberPolicy {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            SharedFolderMemberPolicy::Team => {
                // unit
                let mut s = serializer.serialize_struct("SharedFolderMemberPolicy", 1)?;
                s.serialize_field(".tag", "team")?;
                s.end()
            }
            SharedFolderMemberPolicy::Anyone => {
                // unit
                let mut s = serializer.serialize_struct("SharedFolderMemberPolicy", 1)?;
                s.serialize_field(".tag", "anyone")?;
                s.end()
            }
            SharedFolderMemberPolicy::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

/// Policy governing the visibility of shared links. This policy can apply to newly created shared
/// links, or all shared links.
#[derive(Debug)]
pub enum SharedLinkCreatePolicy {
    /// By default, anyone can access newly created shared links. No login will be required to
    /// access the shared links unless overridden.
    DefaultPublic,
    /// By default, only members of the same team can access newly created shared links. Login will
    /// be required to access the shared links unless overridden.
    DefaultTeamOnly,
    /// Only members of the same team can access all shared links. Login will be required to access
    /// all shared links.
    TeamOnly,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for SharedLinkCreatePolicy {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = SharedLinkCreatePolicy;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a SharedLinkCreatePolicy structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "default_public" => Ok(SharedLinkCreatePolicy::DefaultPublic),
                    "default_team_only" => Ok(SharedLinkCreatePolicy::DefaultTeamOnly),
                    "team_only" => Ok(SharedLinkCreatePolicy::TeamOnly),
                    _ => Ok(SharedLinkCreatePolicy::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["default_public",
                                                    "default_team_only",
                                                    "team_only",
                                                    "other"];
        deserializer.deserialize_struct("SharedLinkCreatePolicy", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for SharedLinkCreatePolicy {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            SharedLinkCreatePolicy::DefaultPublic => {
                // unit
                let mut s = serializer.serialize_struct("SharedLinkCreatePolicy", 1)?;
                s.serialize_field(".tag", "default_public")?;
                s.end()
            }
            SharedLinkCreatePolicy::DefaultTeamOnly => {
                // unit
                let mut s = serializer.serialize_struct("SharedLinkCreatePolicy", 1)?;
                s.serialize_field(".tag", "default_team_only")?;
                s.end()
            }
            SharedLinkCreatePolicy::TeamOnly => {
                // unit
                let mut s = serializer.serialize_struct("SharedLinkCreatePolicy", 1)?;
                s.serialize_field(".tag", "team_only")?;
                s.end()
            }
            SharedLinkCreatePolicy::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

#[derive(Debug)]
pub enum SsoPolicy {
    /// Users will be able to sign in with their Dropbox credentials.
    Disabled,
    /// Users will be able to sign in with either their Dropbox or single sign-on credentials.
    Optional,
    /// Users will be required to sign in with their single sign-on credentials.
    Required,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for SsoPolicy {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = SsoPolicy;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a SsoPolicy structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "disabled" => Ok(SsoPolicy::Disabled),
                    "optional" => Ok(SsoPolicy::Optional),
                    "required" => Ok(SsoPolicy::Required),
                    _ => Ok(SsoPolicy::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["disabled",
                                                    "optional",
                                                    "required",
                                                    "other"];
        deserializer.deserialize_struct("SsoPolicy", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for SsoPolicy {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            SsoPolicy::Disabled => {
                // unit
                let mut s = serializer.serialize_struct("SsoPolicy", 1)?;
                s.serialize_field(".tag", "disabled")?;
                s.end()
            }
            SsoPolicy::Optional => {
                // unit
                let mut s = serializer.serialize_struct("SsoPolicy", 1)?;
                s.serialize_field(".tag", "optional")?;
                s.end()
            }
            SsoPolicy::Required => {
                // unit
                let mut s = serializer.serialize_struct("SsoPolicy", 1)?;
                s.serialize_field(".tag", "required")?;
                s.end()
            }
            SsoPolicy::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

/// Policies governing team members.
#[derive(Debug)]
pub struct TeamMemberPolicies {
    /// Policies governing sharing.
    pub sharing: TeamSharingPolicies,
    /// This describes the Enterprise Mobility Management (EMM) state for this team. This
    /// information can be used to understand if an organization is integrating with a third-party
    /// EMM vendor to further manage and apply restrictions upon the team's Dropbox usage on mobile
    /// devices. This is a new feature and in the future we'll be adding more new fields and
    /// additional documentation.
    pub emm_state: EmmState,
    /// The admin policy around the Dropbox Office Add-In for this team.
    pub office_addin: OfficeAddInPolicy,
}

impl TeamMemberPolicies {
    pub fn new(
        sharing: TeamSharingPolicies,
        emm_state: EmmState,
        office_addin: OfficeAddInPolicy,
    ) -> Self {
        TeamMemberPolicies {
            sharing,
            emm_state,
            office_addin,
        }
    }

}

const TEAM_MEMBER_POLICIES_FIELDS: &'static [&'static str] = &["sharing",
                                                               "emm_state",
                                                               "office_addin"];
impl TeamMemberPolicies {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
    ) -> Result<TeamMemberPolicies, V::Error> {
        use serde::de;
        let mut field_sharing = None;
        let mut field_emm_state = None;
        let mut field_office_addin = None;
        while let Some(key) = map.next_key()? {
            match key {
                "sharing" => {
                    if field_sharing.is_some() {
                        return Err(de::Error::duplicate_field("sharing"));
                    }
                    field_sharing = Some(map.next_value()?);
                }
                "emm_state" => {
                    if field_emm_state.is_some() {
                        return Err(de::Error::duplicate_field("emm_state"));
                    }
                    field_emm_state = Some(map.next_value()?);
                }
                "office_addin" => {
                    if field_office_addin.is_some() {
                        return Err(de::Error::duplicate_field("office_addin"));
                    }
                    field_office_addin = Some(map.next_value()?);
                }
                _ => return Err(de::Error::unknown_field(key, TEAM_MEMBER_POLICIES_FIELDS))
            }
        }
        Ok(TeamMemberPolicies {
            sharing: field_sharing.ok_or_else(|| de::Error::missing_field("sharing"))?,
            emm_state: field_emm_state.ok_or_else(|| de::Error::missing_field("emm_state"))?,
            office_addin: field_office_addin.ok_or_else(|| de::Error::missing_field("office_addin"))?,
        })
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("sharing", &self.sharing)?;
        s.serialize_field("emm_state", &self.emm_state)?;
        s.serialize_field("office_addin", &self.office_addin)
    }
}

impl<'de> ::serde::de::Deserialize<'de> for TeamMemberPolicies {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = TeamMemberPolicies;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a TeamMemberPolicies struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                TeamMemberPolicies::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("TeamMemberPolicies", TEAM_MEMBER_POLICIES_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for TeamMemberPolicies {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("TeamMemberPolicies", 3)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

/// Policies governing sharing within and outside of the team.
#[derive(Debug)]
pub struct TeamSharingPolicies {
    /// Who can join folders shared by team members.
    pub shared_folder_member_policy: SharedFolderMemberPolicy,
    /// Which shared folders team members can join.
    pub shared_folder_join_policy: SharedFolderJoinPolicy,
    /// Who can view shared links owned by team members.
    pub shared_link_create_policy: SharedLinkCreatePolicy,
}

impl TeamSharingPolicies {
    pub fn new(
        shared_folder_member_policy: SharedFolderMemberPolicy,
        shared_folder_join_policy: SharedFolderJoinPolicy,
        shared_link_create_policy: SharedLinkCreatePolicy,
    ) -> Self {
        TeamSharingPolicies {
            shared_folder_member_policy,
            shared_folder_join_policy,
            shared_link_create_policy,
        }
    }

}

const TEAM_SHARING_POLICIES_FIELDS: &'static [&'static str] = &["shared_folder_member_policy",
                                                                "shared_folder_join_policy",
                                                                "shared_link_create_policy"];
impl TeamSharingPolicies {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
    ) -> Result<TeamSharingPolicies, V::Error> {
        use serde::de;
        let mut field_shared_folder_member_policy = None;
        let mut field_shared_folder_join_policy = None;
        let mut field_shared_link_create_policy = None;
        while let Some(key) = map.next_key()? {
            match key {
                "shared_folder_member_policy" => {
                    if field_shared_folder_member_policy.is_some() {
                        return Err(de::Error::duplicate_field("shared_folder_member_policy"));
                    }
                    field_shared_folder_member_policy = Some(map.next_value()?);
                }
                "shared_folder_join_policy" => {
                    if field_shared_folder_join_policy.is_some() {
                        return Err(de::Error::duplicate_field("shared_folder_join_policy"));
                    }
                    field_shared_folder_join_policy = Some(map.next_value()?);
                }
                "shared_link_create_policy" => {
                    if field_shared_link_create_policy.is_some() {
                        return Err(de::Error::duplicate_field("shared_link_create_policy"));
                    }
                    field_shared_link_create_policy = Some(map.next_value()?);
                }
                _ => return Err(de::Error::unknown_field(key, TEAM_SHARING_POLICIES_FIELDS))
            }
        }
        Ok(TeamSharingPolicies {
            shared_folder_member_policy: field_shared_folder_member_policy.ok_or_else(|| de::Error::missing_field("shared_folder_member_policy"))?,
            shared_folder_join_policy: field_shared_folder_join_policy.ok_or_else(|| de::Error::missing_field("shared_folder_join_policy"))?,
            shared_link_create_policy: field_shared_link_create_policy.ok_or_else(|| de::Error::missing_field("shared_link_create_policy"))?,
        })
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("shared_folder_member_policy", &self.shared_folder_member_policy)?;
        s.serialize_field("shared_folder_join_policy", &self.shared_folder_join_policy)?;
        s.serialize_field("shared_link_create_policy", &self.shared_link_create_policy)
    }
}

impl<'de> ::serde::de::Deserialize<'de> for TeamSharingPolicies {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = TeamSharingPolicies;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a TeamSharingPolicies struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                TeamSharingPolicies::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("TeamSharingPolicies", TEAM_SHARING_POLICIES_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for TeamSharingPolicies {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("TeamSharingPolicies", 3)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

#[derive(Debug)]
pub enum TwoStepVerificationPolicy {
    /// Enabled require two factor authorization.
    RequireTfaEnable,
    /// Disabled require two factor authorization.
    RequireTfaDisable,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for TwoStepVerificationPolicy {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = TwoStepVerificationPolicy;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a TwoStepVerificationPolicy structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "require_tfa_enable" => Ok(TwoStepVerificationPolicy::RequireTfaEnable),
                    "require_tfa_disable" => Ok(TwoStepVerificationPolicy::RequireTfaDisable),
                    _ => Ok(TwoStepVerificationPolicy::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["require_tfa_enable",
                                                    "require_tfa_disable",
                                                    "other"];
        deserializer.deserialize_struct("TwoStepVerificationPolicy", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for TwoStepVerificationPolicy {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            TwoStepVerificationPolicy::RequireTfaEnable => {
                // unit
                let mut s = serializer.serialize_struct("TwoStepVerificationPolicy", 1)?;
                s.serialize_field(".tag", "require_tfa_enable")?;
                s.end()
            }
            TwoStepVerificationPolicy::RequireTfaDisable => {
                // unit
                let mut s = serializer.serialize_struct("TwoStepVerificationPolicy", 1)?;
                s.serialize_field(".tag", "require_tfa_disable")?;
                s.end()
            }
            TwoStepVerificationPolicy::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

