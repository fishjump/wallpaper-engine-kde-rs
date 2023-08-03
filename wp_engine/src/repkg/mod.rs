/**
 * This `repkg` module incorporates portions of source code from the `notscuffed/repkg` project.
 *
 * The original source code of `notscuffed/repkg` can be found at the following URL:
https://github.com/notscuffed/repkg
 *
 * We appreciate the authors and contributors of the `notscuffed/repkg` project for their work and inspiration.
 *
 * Please note that, where applicable, the source code from `notscuffed/repkg` is used in compliance with its original license terms.
 *
 * All copyright and license information must be retained, including but not limited to the original author's copyright notice, the license notice, and any original disclaimers.
*/
mod byteorder_ext;
mod package;
mod package_loader;

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;

    use super::package_loader::PackageLoader;

    #[test]
    fn test_package_loader() {
        let file = File::open("../assets/scene.pkg").unwrap();
        let mut reader = BufReader::new(file);
        let loader = PackageLoader::new();

        let pkg = loader.read_from(&mut reader).unwrap();

        assert_eq!(pkg.magic, "PKGV0018");
        assert_eq!(pkg.header_size, 0x1a3);
        assert_eq!(pkg.entries.len(), 10);

        println!("pkg: {:#?}", pkg);
    }
}
