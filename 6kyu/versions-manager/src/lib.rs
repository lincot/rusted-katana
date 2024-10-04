//! <https://www.codewars.com/kata/5bc7bb444be9774f100000c3/train/rust>

mod preloaded;
use digital::{MaxLenBase10, WriteNumUnchecked};
use preloaded::VMError;
use unchecked_std::prelude::*;

#[derive(Clone, Copy)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

pub struct VersionManager {
    versions: Vec<Version>,
}

impl VersionManager {
    pub fn new() -> Self {
        Self {
            versions: vec![Version {
                major: 0,
                minor: 0,
                patch: 1,
            }],
        }
    }

    pub fn from_version(version: &str) -> Result<Self, VMError> {
        if version.is_empty() {
            return Ok(Self::new());
        }

        let dot_pos = version
            .as_bytes()
            .iter()
            .position(|&b| b == b'.')
            .unwrap_or(version.len());
        let major = unsafe { version.get_unchecked(..dot_pos) }
            .parse()
            .map_err(|_| VMError::InvalidVersion)?;
        if dot_pos == version.len() {
            return Ok(Self {
                versions: vec![Version {
                    major,
                    minor: 0,
                    patch: 0,
                }],
            });
        }

        let dot_pos2 = unsafe { version.as_bytes().get_unchecked(dot_pos + 1..) }
            .iter()
            .position(|&b| b == b'.')
            .map_or(version.len(), |pos| pos + dot_pos + 1);
        let minor = unsafe { version.get_unchecked(dot_pos + 1..dot_pos2) }
            .parse()
            .map_err(|_| VMError::InvalidVersion)?;
        if dot_pos2 == version.len() {
            return Ok(Self {
                versions: vec![Version {
                    major,
                    minor,
                    patch: 0,
                }],
            });
        }

        let dot_pos3 = unsafe { version.as_bytes().get_unchecked(dot_pos2 + 1..) }
            .iter()
            .position(|&b| b == b'.')
            .map_or(version.len(), |pos| pos + dot_pos2 + 1);
        let patch = unsafe { version.get_unchecked(dot_pos2 + 1..dot_pos3) }
            .parse()
            .map_err(|_| VMError::InvalidVersion)?;

        Ok(Self {
            versions: vec![Version {
                major,
                minor,
                patch,
            }],
        })
    }

    fn current_version(&self) -> Version {
        *unsafe { self.versions.last().unwrap_unchecked() }
    }

    pub fn major(&mut self) -> &mut Self {
        let mut ver = self.current_version();
        ver.major += 1;
        ver.minor = 0;
        ver.patch = 0;
        self.versions.push(ver);
        self
    }

    pub fn minor(&mut self) -> &mut Self {
        let mut ver = self.current_version();
        ver.minor += 1;
        ver.patch = 0;
        self.versions.push(ver);
        self
    }

    pub fn patch(&mut self) -> &mut Self {
        let mut ver = self.current_version();
        ver.patch += 1;
        self.versions.push(ver);
        self
    }

    pub fn rollback(&mut self) -> Result<&mut Self, VMError> {
        if self.versions.len() <= 1 {
            return Err(VMError::NoHistory);
        }
        self.versions.pop();
        Ok(self)
    }

    pub fn release(&self) -> String {
        let mut res = String::with_capacity(3 * u32::MAX_LEN_BASE10 + 2);
        let Version {
            major,
            minor,
            patch,
        } = self.current_version();
        unsafe {
            res.write_num_unchecked(major, 10, false, false);
            res.push_unchecked('.');
            res.write_num_unchecked(minor, 10, false, false);
            res.push_unchecked('.');
            res.write_num_unchecked(patch, 10, false, false);
        }
        res
    }
}
