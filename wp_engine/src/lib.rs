mod error;
mod greeter;
mod qt_plugin;
mod renderer;
mod vfs;
mod wpscene;

pub mod repkg;

#[cfg(test)]
mod test {
    use crate::vfs::scene_vfs::SceneVFS;
    use crate::wpscene::wp_scene::WPScene;
    use crate::wpscene::wp_scene_object::WPSceneObject;

    #[test]
    fn workflow() {
        // mount assets dir
        let vfs = SceneVFS::new();

        assert!(vfs.mount_dir("", "../assets/wallpaper").is_ok());
        assert!(vfs.exists("/scene.json"));

        let content = vfs.read("/scene.json").unwrap();
        let scene_json = std::str::from_utf8(&content).unwrap();
        println!("{}", scene_json);

        // load scene
        let scene: WPScene = serde_json::from_str(scene_json).unwrap();
        println!("{:#?}", scene);

        // load shader
        for obj in scene.objects {
            match obj {
                WPSceneObject::WPImageObject(image_obj) => {
                    image_obj.effects.iter().for_each(|effect| {
                        let effect_file = vfs
                            .read(format!("/{}", effect.file).as_str())
                            .unwrap();
                        let effect_str = std::str::from_utf8(&effect_file)
                            .unwrap()
                            .to_string();
                        println!("{}", effect_str);
                    });
                }
                WPSceneObject::WPParticleObject(_) => {}
                WPSceneObject::WPSoundObject(_) => {}
                WPSceneObject::WPLightObject(_) => {}
            };
        }
    }
}
