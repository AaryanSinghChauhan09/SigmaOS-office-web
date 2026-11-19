#![forbid(unsafe_code)]
#[derive(Debug, Clone)]
pub struct AssetBundle {
    pub name: String,
    pub urls: Vec<String>,
}

impl AssetBundle {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), urls: Vec::new() } }
    pub fn add(&mut self, url: impl Into<String>) { self.urls.push(url.into()); }
    pub fn count(&self) -> usize { self.urls.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bundle_add() {
        let mut b = AssetBundle::new("core");
        b.add("/app.js");
        assert_eq!(b.count(), 1);
    }
}


impl AssetBundle {
    pub fn contains(&self, url: &str) -> bool {
        self.urls.iter().any(|u| u == url)
    }
    pub fn clear(&mut self) {
        self.urls.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.urls.is_empty()
    }
}

#[cfg(test)]
mod office_web_extra_tests {
    use super::*;

    #[test]
    fn test_bundle_contains_url_aaa() {
        // Arrange
        let mut bundle = AssetBundle::new("scripts");
        bundle.add("/main.js");
        // Act & Assert
        assert!(bundle.contains("/main.js"));
        assert!(!bundle.contains("/style.css"));
    }

    #[test]
    fn test_bundle_clear_aaa() {
        // Arrange
        let mut bundle = AssetBundle::new("styles");
        bundle.add("/style.css");
        assert_eq!(bundle.count(), 1);
        // Act
        bundle.clear();
        // Assert
        assert_eq!(bundle.count(), 0);
        assert!(bundle.is_empty());
    }

    #[test]
    fn test_bundle_empty_initial_state_aaa() {
        // Arrange & Act
        let bundle = AssetBundle::new("empty");
        // Assert
        assert!(bundle.is_empty());
        assert_eq!(bundle.name, "empty");
    }

    #[test]
    fn test_multiple_assets_addition_aaa() {
        // Arrange
        let mut b = AssetBundle::new("fonts");
        // Act
        b.add("/font1.woff2");
        b.add("/font2.woff2");
        // Assert
        assert_eq!(b.count(), 2);
    }
}
