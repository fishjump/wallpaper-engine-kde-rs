use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ScenePath {
    path: String,
}

impl ScenePath {
    pub fn new(path: &str) -> Self {
        ScenePath {
            path: Self::simplify(path),
        }
    }

    fn simplify(path: &str) -> String {
        let mut stack = Vec::new();
        for part in path.split('/') {
            match part {
                ".." => {
                    // if nothing to pop, then it's a relative path
                    // it's fine, no need an error here
                    if stack.len() > 0 {
                        stack.pop();
                    } else {
                        stack.push(part);
                    }
                }
                "." => (),
                _ => stack.push(part),
            }
        }

        stack.join("/")
    }
}

impl Display for ScenePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}
