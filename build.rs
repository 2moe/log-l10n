const EARLY_RT: bool = true;
// const EARLY_RT: bool = false;

use glossa_codegen::{consts::*, prelude::*};
use std::{io, path::PathBuf};

fn main() -> io::Result<()> {
    let ver = get_pkg_version!();
    let rs_path = PathBuf::from_iter(default_l10n_rs_file_arr());

    if is_same_version(&rs_path, Some(ver))? && EARLY_RT {
        return Ok(());
    }
    // append_to_l10n_mod(&rs_path)?;
    //

    let tmp = get_shmem_path(&rs_path)?;
    let mut writer = MapWriter::new(&tmp, &rs_path);

    *writer.get_visibility_mut() = "pub(super)";

    // l10n_path: assets/l10n
    Generator::new(PathBuf::from_iter(default_l10n_dir_arr()))
        .with_version(ver)
        .run(writer)?;

    Ok(())
}
