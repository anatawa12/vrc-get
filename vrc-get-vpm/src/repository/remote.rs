use crate::traits::HttpClient;
use crate::utils::{deserialize_json, deserialize_json_slice};
use crate::version::Version;
use crate::PackageManifest;
use crate::{io, VersionSelector};
use futures::prelude::*;
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::pin::pin;
use url::Url;

type JsonMap = Map<String, Value>;

#[derive(Debug, Clone)]
pub struct RemoteRepository {
    actual: JsonMap,
    parsed: ParsedRepository,
}

#[derive(Deserialize, Debug, Clone)]
struct ParsedRepository {
    #[serde(default)]
    name: Option<Box<str>>,
    #[serde(default)]
    url: Option<Url>,
    #[serde(default)]
    id: Option<Box<str>>,
    #[serde(default)]
    packages: IndexMap<Box<str>, RemotePackages>,
}

impl RemoteRepository {
    pub fn parse(cache: JsonMap) -> io::Result<Self> {
        Ok(Self {
            parsed: deserialize_json(Value::Object(cache.clone()))?,
            actual: cache,
        })
    }

    pub async fn download(
        client: &impl HttpClient,
        url: &Url,
        headers: &IndexMap<Box<str>, Box<str>>,
    ) -> io::Result<(RemoteRepository, Option<Box<str>>)> {
        match Self::download_with_etag(client, url, headers, None).await {
            Ok(None) => unreachable!("downloading without etag should must return Ok(Some)"),
            Ok(Some(repo_and_etag)) => Ok(repo_and_etag),
            Err(err) => Err(err),
        }
    }

    pub async fn download_with_etag(
        client: &impl HttpClient,
        url: &Url,
        headers: &IndexMap<Box<str>, Box<str>>,
        current_etag: Option<&str>,
    ) -> io::Result<Option<(RemoteRepository, Option<Box<str>>)>> {
        let Some((mut stream, etag)) = client.get_with_etag(url, headers, current_etag).await?
        else {
            return Ok(None);
        };

        let mut bytes = Vec::new();
        pin!(stream).read_to_end(&mut bytes).await?;

        let no_bom = bytes
            .strip_prefix(b"\xEF\xBB\xBF")
            .unwrap_or(bytes.as_ref());
        let json = deserialize_json_slice(no_bom)?;

        let mut repo = RemoteRepository::parse(json)?;
        repo.set_url_if_none(|| url.clone());
        Ok(Some((repo, etag)))
    }

    pub(crate) fn set_id_if_none(&mut self, f: impl FnOnce() -> Box<str>) {
        if self.parsed.id.is_none() {
            let id = f();
            self.parsed.id = Some(id.clone());
            self.actual
                .insert("id".to_owned(), Value::String(id.into()));
        }
    }

    pub(crate) fn set_url_if_none(&mut self, f: impl FnOnce() -> Url) {
        if self.parsed.url.is_none() {
            let url = f();
            self.parsed.url = Some(url.clone());
            self.actual
                .insert("url".to_owned(), Value::String(url.to_string()));
            if self.parsed.id.is_none() {
                let url = self.parsed.url.as_ref().unwrap().as_str().into();
                self.set_id_if_none(move || url);
            }
        }
    }

    pub fn url(&self) -> Option<&Url> {
        self.parsed.url.as_ref()
    }

    pub fn id(&self) -> Option<&str> {
        self.parsed.id.as_deref()
    }

    pub fn name(&self) -> Option<&str> {
        self.parsed.name.as_deref()
    }

    pub fn get_versions_of(&self, package: &str) -> impl Iterator<Item = &'_ PackageManifest> {
        self.parsed
            .packages
            .get(package)
            .map(RemotePackages::all_versions)
            .into_iter()
            .flatten()
    }

    pub fn get_packages(&self) -> impl Iterator<Item = &'_ RemotePackages> {
        self.parsed.packages.values()
    }

    pub fn get_package_version(&self, name: &str, version: &Version) -> Option<&PackageManifest> {
        self.parsed.packages.get(name)?.versions.get(version)
    }
}

impl Serialize for RemoteRepository {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.actual.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RemoteRepository {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let map = JsonMap::deserialize(deserializer)?;
        Self::parse(map).map_err(Error::custom)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct RemotePackages {
    #[serde(default)]
    versions: HashMap<Version, PackageManifest>,
}

impl RemotePackages {
    pub fn all_versions(&self) -> impl Iterator<Item = &PackageManifest> {
        self.versions.values()
    }

    pub fn get_latest_may_yanked(&self, selector: VersionSelector) -> Option<&PackageManifest> {
        self.get_latest(selector).or_else(|| {
            self.versions
                .values()
                .filter(|json| selector.satisfies(json))
                .max_by_key(|json| json.version())
        })
    }

    pub fn get_latest(&self, selector: VersionSelector) -> Option<&PackageManifest> {
        self.versions
            .values()
            .filter(|json| selector.satisfies(json))
            .clone()
            .filter(|json| !json.is_yanked())
            .max_by_key(|json| json.version())
    }
}
