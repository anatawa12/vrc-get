use crate::bson::{DateTime, ObjectId};
use crate::lowlevel::{FFISlice, FromFFI, ToFFI};
use std::fmt::{Debug, Formatter};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ProjectType(u32);

impl ProjectType {
    pub const UNKNOWN: Self = Self(0);
    pub const LEGACY_SDK2: Self = Self(1);
    pub const LEGACY_WORLDS: Self = Self(2);
    pub const LEGACY_AVATARS: Self = Self(3);
    pub const UPM_WORLDS: Self = Self(4);
    pub const UPM_AVATARS: Self = Self(5);
    pub const UPM_STARTER: Self = Self(6);
    pub const WORLDS: Self = Self(7);
    pub const AVATARS: Self = Self(8);
    pub const VPM_STARTER: Self = Self(9);
}

impl Debug for ProjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProjectType::UNKNOWN => f.write_str("Unknown"),
            ProjectType::LEGACY_SDK2 => f.write_str("Legacy SDK2"),
            ProjectType::LEGACY_WORLDS => f.write_str("Legacy Worlds"),
            ProjectType::LEGACY_AVATARS => f.write_str("Legacy Avatars"),
            ProjectType::UPM_WORLDS => f.write_str("UPM Worlds"),
            ProjectType::UPM_AVATARS => f.write_str("UPM Avatars"),
            ProjectType::UPM_STARTER => f.write_str("UPM Starter"),
            ProjectType::WORLDS => f.write_str("Worlds"),
            ProjectType::AVATARS => f.write_str("Avatars"),
            ProjectType::VPM_STARTER => f.write_str("VPM Starter"),
            _ => f.write_fmt(format_args!("Unexpected({})", self.0)),
        }
    }
}

/// Represents a VCC Project
#[derive(Debug)]
pub struct Project {
    path: Box<str>,
    unity_version: Option<Box<str>>,
    created_at: DateTime,
    last_modified: DateTime,
    type_: ProjectType,
    id: ObjectId,
    favorite: bool,
}

impl Project {
    pub fn new(path: Box<str>, unity_version: Option<Box<str>>, project_type: ProjectType) -> Self {
        let created_at: DateTime = DateTime::now();

        Self {
            path,
            unity_version,
            created_at,
            last_modified: created_at,
            type_: project_type,
            id: ObjectId::new(),
            favorite: false,
        }
    }

    pub fn id(&self) -> ObjectId {
        self.id
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn project_type(&self) -> ProjectType {
        self.type_
    }

    pub fn unity_version(&self) -> Option<&str> {
        self.unity_version.as_deref()
    }

    pub fn favorite(&self) -> bool {
        self.favorite
    }

    pub fn created_at(&self) -> DateTime {
        self.created_at
    }

    pub fn last_modified(&self) -> DateTime {
        self.last_modified
    }

    pub fn set_path(&mut self, path: Box<str>) {
        self.path = path;
    }

    pub fn set_project_type(&mut self, project_type: ProjectType) {
        self.type_ = project_type;
    }

    pub fn set_unity_version(&mut self, unity_version: Option<Box<str>>) {
        self.unity_version = unity_version;
    }

    pub fn set_favorite(&mut self, favorite: bool) {
        self.favorite = favorite;
    }

    pub fn set_last_modified(&mut self, last_modified: DateTime) {
        self.last_modified = last_modified;
    }
}

#[repr(C)]
pub(crate) struct ProjectFFI {
    path: FFISlice,
    unity_version: FFISlice,
    created_at: DateTime,
    last_modified: DateTime,
    type_: ProjectType,
    id: ObjectId,
    favorite: u8,
}

impl FromFFI for Project {
    type FFIType = ProjectFFI;

    unsafe fn from_ffi(ffi: ProjectFFI) -> Self {
        Self {
            path: FromFFI::from_ffi(ffi.path),
            unity_version: FromFFI::from_ffi(ffi.unity_version),
            created_at: ffi.created_at,
            last_modified: ffi.last_modified,
            type_: ffi.type_,
            id: ffi.id,
            favorite: ffi.favorite != 0,
        }
    }
}

impl ToFFI for Project {
    type FFIType = ProjectFFI;

    unsafe fn to_ffi(&self) -> Self::FFIType {
        ProjectFFI {
            path: self.path.as_ref().to_ffi(),
            unity_version: self.unity_version.as_deref().to_ffi(),
            created_at: self.created_at,
            last_modified: self.last_modified,
            type_: self.type_,
            id: self.id,
            favorite: self.favorite as u8,
        }
    }
}
