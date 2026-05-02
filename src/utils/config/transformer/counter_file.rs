use std::path::Path;

use crate::utils::{
    cache::BinaryCache,
    errors::types::{FileAction, SherlockErrorType},
};
use std::collections::HashMap;

pub fn transform_counter_file(path: &Path) {
    let is_u16 =
        std::panic::catch_unwind(|| BinaryCache::read::<HashMap<String, u16>, _>(path).is_ok())
            .unwrap_or(false);

    if is_u16 {
        eprintln!("[Counter File]: Already correct format, skipping");
        return;
    }

    let old: HashMap<String, u32> = match BinaryCache::read(path) {
        Ok(data) => data,
        Err(e)
            if matches!(
                e.error_type,
                SherlockErrorType::FileError(FileAction::Find, _)
            ) =>
        {
            eprintln!(
                "[Counter File]: Could not find counter file. Checked: {:?}",
                path
            );
            return;
        }
        Err(e) => {
            eprintln!(
                "[Counter File]: Failed to parse initial file as <String, u32>. Caused by: {:?}",
                e.traceback
            );
            return;
        }
    };

    let new: HashMap<String, u16> = old
        .into_iter()
        .map(|(name, count)| (name, count as u16))
        .collect();

    if let Err(e) = BinaryCache::write(path, &new) {
        eprintln!(
            "[Counter File]: Failed to write new file. Error during serialization: {:?}",
            e
        );
    }
}
