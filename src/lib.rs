//! # Squashfs-deku
//! Read and extract information from a Squashfs image

pub mod compressor;
pub mod dir;
pub mod error;
pub mod fragment;
pub mod inode;
pub mod metadata;
pub mod squashfs;

pub use compressor::{CompressionOptions, Compressor};
pub use dir::{Dir, DirEntry};
pub use fragment::Fragment;
pub use inode::{BasicDirectory, BasicFile, Inode};
pub use metadata::Metadata;
pub use squashfs::Squashfs;
