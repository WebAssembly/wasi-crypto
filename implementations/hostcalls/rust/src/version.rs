#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Version(pub u64);

impl Version {
    pub const UNSPECIFIED: Version = Version(0xff00_0000_0000_0000);
    pub const LATEST: Version = Version(0xff00_0000_0000_0000);
    pub const ALL: Version = Version(0xff00_0000_0000_0000);
}
