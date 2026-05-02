use glob::Pattern;
use rayon::prelude::*;
use simd_json;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use super::utils::{AppData, SherlockAlias};
use crate::launcher::Launcher;
use crate::loader::application_loader::parser::DesktopFileParser;
use crate::prelude::PathHelpers;
use crate::sherlock_msg;
use crate::utils::cache::BinaryCache;
use crate::utils::errors::types::{FileAction, SherlockErrorType};
use crate::utils::files::{expand_path, home_dir};
use crate::utils::{config::ConfigGuard, errors::SherlockMessage, files::read_lines};

mod parser;

pub struct ApplicationLoader;
impl ApplicationLoader {
    ////// Loads and synchronizes the application registry from disk and cache.
    ///
    /// Uses following paths as a reference for the `.desktop` files:
    /// * XDG_DATA_HOME (or `~/.local/share/`)
    /// * XDG_DATA_DIRS (or `/usr/share/applications/` and `/usr/local/share/`),
    /// * User-specified locations using `SherlockConfig.debug.app_paths`
    ///
    /// Update Strategy:
    /// 1. **Discovery**: Identifies `.desktop` files modified since the last cache write.
    /// 2. **Differential Loading**: Only parses new or changed files from disk, while
    ///    recycling valid entries from the `BinaryCache`.
    /// 3. **Smart Persistence**: Spawns a background task to update the cache if
    ///    any discrepancies (stale entries or new files) are detected.
    ///
    /// Returns an `Arc<Vec<AppData>>` to allow for zero-copy sharing across threads.
    /// Downstream consumers can use `Arc::unwrap_or_clone()` to obtain a mutable
    /// `Vec` efficiently—performing a deep copy only if the data is currently
    /// shared with a background write-task.
    ///
    /// # Errors
    /// Returns a `SherlockMessage` if configuration is unreachable or disk parsing
    /// fails critically.
    pub fn load_applications(
        launcher: Arc<Launcher>,
        counts: &HashMap<String, u32>,
        decimals: i32,
        use_keywords: bool,
    ) -> Result<Arc<Vec<AppData>>, SherlockMessage> {
        let config = ConfigGuard::read()?;
        let cache_path: Arc<Path> = config.caching.cache.as_path().into();

        // forces update in case mtime is somehow not availalbe (pretty impossible)
        let last_cached = cache_path.modtime().unwrap_or(SystemTime::UNIX_EPOCH);
        let need_update = Self::get_new_apps(last_cached);
        let update_lookup = LookupCache::new(&need_update);

        let new_apps = Self::load_applications_from_disk(
            need_update.clone(),
            &launcher,
            counts,
            decimals,
            use_keywords,
        )?;

        let Ok(cached_apps) = BinaryCache::read::<Vec<AppData>, _>(&config.caching.cache) else {
            let apps: Arc<Vec<AppData>> = new_apps.into();
            Self::spawn_cache_write(&cache_path, apps.clone());
            return Ok(apps);
        };
        let cached_apps_len = cached_apps.len();

        let all_apps: Arc<Vec<AppData>> = cached_apps
            .into_iter()
            .filter(|data| {
                data.desktop_file
                    .as_ref()
                    .is_some_and(|d| d.exists() && !update_lookup.contains(d))
            })
            .map(|mut ad| {
                ad.priority = ad.get_exec(&launcher).map(|exec| {
                    parse_priority(
                        launcher.priority as f32,
                        counts.get(&exec).copied().unwrap_or(0u32),
                        decimals,
                    )
                });
                ad
            })
            .chain(new_apps)
            .collect::<Vec<_>>()
            .into();

        let cache_is_stale = !need_update.is_empty() || all_apps.len() != cached_apps_len;
        if cache_is_stale {
            Self::spawn_cache_write(&cache_path, all_apps.clone());
        }

        Ok(all_apps)
    }

    #[inline]
    pub fn get_new_apps(since: SystemTime) -> Arc<[PathBuf]> {
        Self::get_applications_dir()
            .iter()
            .flat_map(|p| p.read_dir().into_iter().flatten())
            .filter_map(Result::ok)
            .filter_map(|e| {
                let path = e.path();
                let is_desktop = path
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("desktop"));
                let is_new = path.modtime().is_some_and(|t| t > since);
                (is_desktop && is_new).then_some(path)
            })
            .collect()
    }

    fn load_applications_from_disk(
        files: Arc<[PathBuf]>,
        launcher: &Arc<Launcher>,
        counts: &HashMap<String, u32>,
        decimals: i32,
        use_keywords: bool,
    ) -> Result<Vec<AppData>, SherlockMessage> {
        if files.is_empty() {
            return Ok(vec![]);
        }

        let ignore;
        let aliases;
        {
            let config = ConfigGuard::read()?;
            ignore = Self::load_ignore_patterns(&config.files.ignore)?;
            aliases = RwLock::new(Self::load_aliases(&config.files.alias)?);
        }
        let parser = DesktopFileParser::new(launcher, &ignore, counts, decimals, use_keywords);

        let apps: Vec<AppData> = files
            .into_par_iter()
            .filter_map(|path| parser.parse(path, &aliases))
            .collect();

        Ok(apps)
    }

    #[inline(always)]
    fn spawn_cache_write(cache_path: &Path, apps: Arc<Vec<AppData>>) {
        let path: Arc<Path> = cache_path.into();
        rayon::spawn_fifo(move || {
            if let Err(e) = BinaryCache::write(&path, &apps) {
                eprintln!("{}", e.error_type);
            }
        });
    }

    #[inline(always)]
    fn load_ignore_patterns(path: &Path) -> Result<Vec<Pattern>, SherlockMessage> {
        match read_lines(&path) {
            Ok(lines) => Ok(lines
                .map_while(Result::ok)
                .filter_map(|l| Pattern::new(&l.to_lowercase()).ok())
                .collect()),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(Vec::new()),
            Err(e) => Err(sherlock_msg!(
                Warning,
                SherlockErrorType::FileError(FileAction::Read, path.to_path_buf()),
                e
            )),
        }
    }

    #[inline(always)]
    fn load_aliases(path: &Path) -> Result<HashMap<String, SherlockAlias>, SherlockMessage> {
        match File::open(&path) {
            Ok(f) => simd_json::from_reader(f)
                .map_err(|e| sherlock_msg!(Warning, SherlockErrorType::DeserializationError, e)),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(HashMap::new()),
            Err(e) => Err(sherlock_msg!(
                Warning,
                SherlockErrorType::FileError(FileAction::Read, path.to_path_buf()),
                e
            )),
        }
    }

    pub fn get_applications_dir() -> Arc<[PathBuf]> {
        let home = home_dir().unwrap_or_default();
        let xdg_data_home = env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join(".local/share"));

        let xdg_data_dirs =
            env::var("XDG_DATA_DIRS").unwrap_or_else(|_| "/usr/local/share:/usr/share".into());

        let xdg_paths = xdg_data_dirs
            .split(':')
            .map(|p| PathBuf::from(p).join("applications"))
            .chain(std::iter::once(xdg_data_home));

        match ConfigGuard::read() {
            Ok(c) if !c.debug.app_paths.is_empty() => xdg_paths
                .chain(c.debug.app_paths.iter().map(|p| expand_path(p, &home)))
                .filter(|p| p.exists())
                .collect(),
            _ => xdg_paths.filter(|p| p.exists()).collect(),
        }
    }

    pub fn get_desktop_files() -> Arc<[PathBuf]> {
        Self::get_applications_dir()
            .iter()
            .flat_map(|p| p.read_dir().into_iter().flatten())
            .filter_map(Result::ok)
            .filter_map(|e| {
                let path = e.path();
                let is_desktop = path
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("desktop"));
                is_desktop.then_some(path)
            })
            .collect()
    }
}

fn should_ignore(ignore_apps: &[Pattern], app: &str) -> bool {
    let app_name = app.to_lowercase();
    ignore_apps.iter().any(|pattern| pattern.matches(&app_name))
}
pub fn parse_priority(priority: f32, count: u32, decimals: i32) -> f32 {
    priority + 0.99 - count as f32 * 10f32.powi(-decimals)
}

pub fn file_has_changed(file_path: &Path, compare_to: &Path) -> bool {
    match (&file_path.modtime(), &compare_to.modtime()) {
        (Some(t1), Some(t2)) if t1 > t2 => true, // t1 is newer than t2
        (Some(t1), Some(t2)) if t1 < t2 => false, // t1 is older than t2
        _ => true,                               // if there is a modtime missing
    }
}

impl PathHelpers for Path {
    fn modtime(&self) -> Option<SystemTime> {
        let meta = self.metadata().ok()?;
        meta.modified().or_else(|_| meta.created()).ok()
    }
}

/// Adaptive lookup structure that selects the most efficient contains-check
/// strategy based on the number of elements.
///
/// On most systems, only a handful of `.desktop` files change at once, so a
/// linear slice scan is faster due to cache locality and zero hashing overhead.
/// For larger sets — such as first install, cache clearing, or after a large
/// package update — a [`HashSet`] is used instead for O(1) lookups.
pub enum LookupCache<'a> {
    Set(HashSet<&'a PathBuf>),
    Slice(&'a [PathBuf]),
}

impl<'a> LookupCache<'a> {
    pub fn new(input: &'a Arc<[PathBuf]>) -> Self {
        if input.len() > 25 {
            Self::Set(input.iter().collect())
        } else {
            Self::Slice(input)
        }
    }

    pub fn contains(&self, path: &PathBuf) -> bool {
        match self {
            Self::Set(s) => s.contains(path),
            Self::Slice(v) => v.contains(path),
        }
    }
}
