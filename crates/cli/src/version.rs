use semver::Version;

pub trait VersionExt {
  fn is_zero(&self) -> bool;
}

impl VersionExt for Version {
  fn is_zero(&self) -> bool {
    self.major == 0 && self.minor == 0 && self.patch == 0
  }
}
