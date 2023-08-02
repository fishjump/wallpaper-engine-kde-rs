mod scene_file;
mod scene_path;
pub mod scene_vfs;
pub mod scene_vfs_error;

#[cfg(test)]
mod test {
    use crate::vfs::scene_vfs::SceneVFS;

    #[test]
    fn basic() {
        let vfs = SceneVFS::new();

        assert!(!vfs.exists("readme.txt").unwrap());
        assert!(vfs.write("readme.txt", "hello,world").is_ok());
        assert!(vfs.exists("readme.txt").unwrap());

        let content = vfs.read("readme.txt").unwrap();
        let new_str = std::str::from_utf8(&content).unwrap();
        assert_eq!(new_str, "hello,world");
    }

    #[test]
    fn link_and_fetch() {
        let vfs = SceneVFS::new();

        assert!(!vfs.exists("Cargo.toml").unwrap());
        assert!(vfs.link("./Cargo.toml", "Cargo.toml").is_ok());

        assert!(vfs.exists("Cargo.toml").unwrap());

        let content = vfs.read("Cargo.toml").unwrap();
        let from_vfs = std::str::from_utf8(&content).unwrap();
        assert!(!from_vfs.is_empty());

        let from_local = std::fs::read_to_string("./Cargo.toml").unwrap();

        assert_eq!(from_vfs, from_local.as_str());
    }
}
