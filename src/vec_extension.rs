use std::path::PathBuf;

pub trait PathBufVecExt {
    fn find_file_with_substring(&self, substring: &str) -> Option<&PathBuf>;
}

impl PathBufVecExt for Vec<PathBuf> {
    fn find_file_with_substring(&self, substring: &str) -> Option<&PathBuf> {
        self.iter().find(|file| {
            file.to_str()
                .and_then(|s| Some(s.contains(substring)))
                .unwrap_or(false)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    static FILES: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
        return vec![
            PathBuf::from("document.txt"),
            PathBuf::from("image1.jpg"),
            PathBuf::from("image2.jpg"),
        ];
    });

    #[test]
    fn find_file_with_substring_return_the_some_value_if_match() {
        assert_eq!(
            FILES.find_file_with_substring("image1"),
            Some(&PathBuf::from("image1.jpg"))
        );
    }

    #[test]
    fn find_file_with_substring_return_the_first_match() {
        assert_eq!(
            FILES.find_file_with_substring("image"),
            Some(&PathBuf::from("image1.jpg"))
        );
    }

    #[test]
    fn find_file_with_substring_return_none_if_not_find_a_match() {
        assert_eq!(FILES.find_file_with_substring("notExisting"), None);
    }
}
