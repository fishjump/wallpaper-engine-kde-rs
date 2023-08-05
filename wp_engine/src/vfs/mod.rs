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

        assert!(!vfs.exists("readme.txt"));
        vfs.write("readme.txt", "hello,world");
        assert!(vfs.exists("readme.txt"));

        let content = vfs.read("readme.txt").unwrap();
        let new_str = std::str::from_utf8(&content).unwrap();
        assert_eq!(new_str, "hello,world");
    }

    #[test]
    fn mount_and_fetch() {
        let vfs = SceneVFS::new();

        assert!(!vfs.exists("Cargo.toml"));
        vfs.mount("Cargo.toml", "./Cargo.toml");
        assert!(vfs.exists("Cargo.toml"));

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
        assert!(!vfs.exists("src"));
        assert!(vfs.mount_dir("/wp_engine", ".").is_ok());
        assert!(vfs.exists("/wp_engine/Cargo.toml"));
        assert!(vfs.exists("/wp_engine/Makefile.toml"));
        assert!(vfs.exists("/wp_engine/src/lib.rs"));
    }

    #[test]
    fn path_simplify() {
        let path1 = ScenePath::new("/path/to/file");
        assert_eq!(path1.to_string(), "/path/to/file");

        let path2 = ScenePath::new("/path/to/./file/../../to/file");
        assert_eq!(path2.to_string(), "/path/to/file");

        assert_eq!(path1, path2);
    }
}
