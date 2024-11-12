use bevy::{asset::AssetPath, prelude::*};

/// Enum that represents either a handle or an owned path.
///
/// This is useful for when you want to specify an asset, but also want it to be have lifetime 'static
#[derive(Clone, Debug)]
pub enum HandleOrOwnedPath<T: Asset> {
    Handle(Handle<T>),
    Path(String),
}

impl<T: Asset> Default for HandleOrOwnedPath<T> {
    fn default() -> Self {
        Self::Path("".to_string())
    }
}

// Necessary because we don't want to require T: PartialEq
impl<T: Asset> PartialEq for HandleOrOwnedPath<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HandleOrOwnedPath::Handle(h1), HandleOrOwnedPath::Handle(h2)) => h1 == h2,
            (HandleOrOwnedPath::Path(p1), HandleOrOwnedPath::Path(p2)) => p1 == p2,
            _ => false,
        }
    }
}

impl<T: Asset> From<Handle<T>> for HandleOrOwnedPath<T> {
    fn from(h: Handle<T>) -> Self {
        HandleOrOwnedPath::Handle(h)
    }
}

impl<T: Asset> From<&str> for HandleOrOwnedPath<T> {
    fn from(p: &str) -> Self {
        HandleOrOwnedPath::Path(p.to_string())
    }
}

impl<T: Asset> From<String> for HandleOrOwnedPath<T> {
    fn from(p: String) -> Self {
        HandleOrOwnedPath::Path(p.clone())
    }
}

impl<T: Asset> From<&String> for HandleOrOwnedPath<T> {
    fn from(p: &String) -> Self {
        HandleOrOwnedPath::Path(p.to_string())
    }
}

impl<T: Asset + Clone> From<&HandleOrOwnedPath<T>> for HandleOrOwnedPath<T> {
    fn from(p: &HandleOrOwnedPath<T>) -> Self {
        p.to_owned()
    }
}

/// Enum that represents either a handle or an asset path or nothing.
#[derive(Clone, Default, Debug)]
pub enum MaybeHandleOrPath<'a, T: Asset> {
    #[default]
    None,
    Handle(Handle<T>),
    Path(AssetPath<'a>),
}

// Necessary because we don't want to require T: PartialEq
impl<'a, T: Asset> PartialEq for MaybeHandleOrPath<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MaybeHandleOrPath::None, MaybeHandleOrPath::None) => true,
            (MaybeHandleOrPath::Handle(h1), MaybeHandleOrPath::Handle(h2)) => h1 == h2,
            (MaybeHandleOrPath::Path(p1), MaybeHandleOrPath::Path(p2)) => p1 == p2,
            _ => false,
        }
    }
}

impl<T: Asset> From<Handle<T>> for MaybeHandleOrPath<'_, T> {
    fn from(h: Handle<T>) -> Self {
        MaybeHandleOrPath::Handle(h)
    }
}

impl<'a, T: Asset> From<AssetPath<'a>> for MaybeHandleOrPath<'a, T> {
    fn from(p: AssetPath<'a>) -> Self {
        MaybeHandleOrPath::Path(p)
    }
}

impl<'a, T: Asset> From<&'a str> for MaybeHandleOrPath<'a, T> {
    fn from(p: &'a str) -> Self {
        MaybeHandleOrPath::Path(AssetPath::parse(p))
    }
}

impl<'a, T: Asset> From<Option<AssetPath<'a>>> for MaybeHandleOrPath<'a, T> {
    fn from(p: Option<AssetPath<'a>>) -> Self {
        match p {
            Some(p) => MaybeHandleOrPath::Path(p),
            None => MaybeHandleOrPath::None,
        }
    }
}

impl<'a, T: Asset> From<&'a HandleOrOwnedPath<T>> for MaybeHandleOrPath<'a, T> {
    fn from(p: &'a HandleOrOwnedPath<T>) -> Self {
        match p {
            HandleOrOwnedPath::Handle(h) => MaybeHandleOrPath::Handle(h.clone()),
            HandleOrOwnedPath::Path(p) => MaybeHandleOrPath::Path(AssetPath::parse(p)),
        }
    }
}
