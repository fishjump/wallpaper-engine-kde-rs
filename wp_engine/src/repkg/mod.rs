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
mod tex;

pub use package::package::Package;
pub use tex::tex::Tex;

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;

    use crate::repkg::{Package, Tex};

    #[test]
    fn load_package() {
        let file = File::open("../assets/scene.pkg").unwrap();
        let mut reader = BufReader::new(file);

        let pkg = Package::read_from(&mut reader).unwrap();

        assert_eq!(pkg.magic, "PKGV0018");
        assert_eq!(pkg.header_size, 0x1a3);
        assert_eq!(pkg.entries.len(), 10);

        println!("pkg: {:#?}", pkg);
    }

    #[test]
    fn load_tex() {
        let file =
            File::open("../assets/wallpaper/materials/00009.tex").unwrap();
        let mut reader = BufReader::new(file);
        let tex = Tex::read_from(&mut reader).unwrap();

        assert_eq!(tex.magic1, "TEXV0005");
        assert_eq!(tex.magic2, "TEXI0001");
        assert_eq!(tex.header.tex_width, 4096);
        assert_eq!(tex.header.tex_height, 2048);
        assert_eq!(tex.header.img_width, 2560);
        assert_eq!(tex.header.img_height, 1440);

        println!("tex: {:#?}", tex);
    }
}
