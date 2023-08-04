mod scene_file;
mod scene_path;
pub mod scene_vfs;

#[cfg(test)]
mod test {
    use crate::vfs::scene_path::ScenePath;
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
    fn mount_and_fetch() {
        let vfs = SceneVFS::new();

        assert!(!vfs.exists("Cargo.toml").unwrap());
        assert!(vfs.mount("Cargo.toml", "./Cargo.toml").is_ok());
        assert!(vfs.exists("Cargo.toml").unwrap());

        let content = vfs.read("Cargo.toml").unwrap();
        let from_vfs = std::str::from_utf8(&content).unwrap();
        assert!(!from_vfs.is_empty());

        let from_local = std::fs::read_to_string("./Cargo.toml").unwrap();

        assert_eq!(from_vfs, from_local.as_str());
    }

    #[test]
    fn mount_dir() {
        let vfs = SceneVFS::new();

        // "~/steam/common/.local/share/Steam/steamapps/common/wallpaper_engine/assets"
        // Issue:
        // 1. dont support home dir, e.g., ~/xxx
        // 2. dont support relative path from parent dir, e.g., ../xxx (because of the ScenePath impl)
        assert!(!vfs.exists("src").unwrap());
        assert!(vfs.mount_dir("/wp_engine", ".").is_ok());
        assert!(vfs.exists("/wp_engine/Cargo.toml").unwrap());
        assert!(vfs.exists("/wp_engine/Makefile.toml").unwrap());
        assert!(vfs.exists("/wp_engine/src/lib.rs").unwrap());
    }

    #[test]
    fn path_simplify() {
        let path1 = ScenePath::new("/path/to/file").unwrap();
        assert_eq!(path1.to_string(), "/path/to/file");

        let path2 = ScenePath::new("/path/to/./file/../../to/file").unwrap();
        assert_eq!(path2.to_string(), "/path/to/file");

        assert_eq!(path1, path2);
    }
}
