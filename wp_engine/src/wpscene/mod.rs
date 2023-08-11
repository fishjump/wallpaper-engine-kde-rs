use serde::de::Error;
use serde::{Deserialize, Deserializer};

pub mod wp_scene;
pub mod wp_scene_camera;
pub mod wp_scene_general;
pub mod wp_scene_object;

pub(super) fn from_str_to_arr3<'de, D>(
    deserializer: D,
) -> Result<[f32; 3], D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let vec = from_str_to_arr::<D>(&s, 3)?;

    Ok([vec[0], vec[1], vec[2]])
}

pub(super) fn from_str_to_arr2<'de, D>(
    deserializer: D,
) -> Result<[f32; 2], D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let vec = from_str_to_arr::<D>(&s, 2)?;

    Ok([vec[0], vec[1]])
}

fn from_str_to_arr<'de, D>(s: &str, len: usize) -> Result<Vec<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec: Result<Vec<f32>, _> =
        s.split_whitespace().map(|x| x.parse()).collect();

    let vec: Result<Vec<f32>, D::Error> =
        vec.map_err(|err| Error::custom(format!("{:#?}", err)));

    if let Err(err) = vec {
        return Err(err);
    }

    let vec = vec.unwrap();
    if vec.len() != len {
        return Err(Error::custom(format!(
                "Data count mismatch, string: {}, expect length: 3, actual data: {:?}",
                s, vec
            )));
    }

    Ok(vec)
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;

    use crate::wpscene::wp_scene::WPScene;

    #[test]
    fn wp_scene() {
        // let mut f = File::open("../assets/wallpaper/scene.json").unwrap();
        let mut f = File::open("/mnt/c/Program Files (x86)/Steam/steamapps/workshop/content/431960/2619088810/output/scene.json").unwrap();

        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();

        let scene: Result<WPScene, _> = serde_json::from_str(buf.as_str());
        assert!(scene.is_ok());

        println!("{:#?}", scene.unwrap());
    }
}
