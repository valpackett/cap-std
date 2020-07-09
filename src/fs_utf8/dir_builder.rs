use std::fs;

/// A builder used to create directories in various manners.
///
/// This corresponds to [`std::fs::DirBuilder`].
///
/// Unlike `std::fs::DirBuilder`, this API has no `DirBuilder::create`, because
/// creating directories requires a capability. Use
/// [`Dir::create_with_dir_builder`] instead.
///
/// [`std::fs::DirBuilder`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html
/// [`Dir::create_with_dir_builder`]: https://doc.rust-lang.org/std/fs/struct.Dir.html#method.create_with_dir_builder
///
/// <details>
/// We need to define our own version because the libstd `DirBuilder` doesn't have
/// public accessors that we can use.
/// </details>
pub struct DirBuilder {
    pub(crate) cap_std: crate::fs::DirBuilder,
}

impl DirBuilder {
    /// Constructs a new instance of `Self` from the given `std::fs::File`.
    #[inline]
    pub const fn from_std(std: fs::DirBuilder) -> Self {
        Self::from_cap_std(crate::fs::DirBuilder::from_std(std))
    }

    /// Constructs a new instance of `Self` from the given `cap_std::fs::File`.
    #[inline]
    pub const fn from_cap_std(cap_std: crate::fs::DirBuilder) -> Self {
        Self { cap_std }
    }

    /// Creates a new set of options with default mode/security settings for all platforms and also non-recursive.
    ///
    /// This corresponds to [`std::fs::DirBuilder::new`].
    ///
    /// [`std::fs::DirBuilder::new`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html#method.new
    #[allow(clippy::new_without_default)]
    #[inline]
    pub fn new() -> Self {
        Self {
            cap_std: crate::fs::DirBuilder::new(),
        }
    }

    /// Indicates that directories should be created recursively, creating all parent directories.
    ///
    /// This corresponds to [`std::fs::DirBuilder::recursive`].
    ///
    /// [`std::fs::DirBuilder::recursive`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html#method.recursive
    #[inline]
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.cap_std.recursive(recursive);
        self
    }
}

#[cfg(unix)]
impl std::os::unix::fs::DirBuilderExt for DirBuilder {
    #[inline]
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.cap_std.mode(mode);
        self
    }
}

// TODO: impl Debug for DirBuilder? But don't expose DirBuilder's path...