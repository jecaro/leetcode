pub fn simplify_path(path: String) -> String {
    let result: String = path
        .split('/')
        .fold(Vec::new(), |mut acc, dir| {
            match dir {
                "" | "." => (),
                ".." => {
                    acc.pop();
                    ()
                }
                _ => acc.push(dir),
            }
            acc
        })
        .iter()
        .map(|dir| String::from("/") + *dir)
        .collect();
    if result.is_empty() {
        String::from("/")
    } else {
        result
    }
}

#[cfg(test)]
mod simplify_path {

    #[test]
    fn example1() {
        let path = String::from("/home/");
        let result = super::simplify_path(path);
        assert_eq!(result, String::from("/home"));
    }

    #[test]
    fn example2() {
        let path = String::from("/home//foo/");
        let result = super::simplify_path(path);
        assert_eq!(result, String::from("/home/foo"));
    }

    #[test]
    fn example3() {
        let path = String::from("/home/user/Documents/../Pictures");
        let result = super::simplify_path(path);
        assert_eq!(result, String::from("/home/user/Pictures"));
    }

    #[test]
    fn example4() {
        let path = String::from("/../");
        let result = super::simplify_path(path);
        assert_eq!(result, String::from("/"));
    }

    #[test]
    fn example5() {
        let path = String::from("/.../a/../b/c/../d/./");
        let result = super::simplify_path(path);
        assert_eq!(result, String::from("/.../b/d"));
    }
}
