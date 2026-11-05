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
