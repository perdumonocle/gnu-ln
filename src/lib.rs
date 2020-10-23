use std::error::Error;
use std::path::Path;
use std::process::Command;

pub enum BackupControl {
    None,
    Off,
    Numbered,
    T,
    Existing,
    Nil,
    Simple,
    Never,
}

#[derive(Default)]
pub struct LnOptions<'a> {
    backup: Option<BackupControl>,
    b: bool,
    directory: bool,
    force: bool,
    logical: bool,
    no_dereference: bool,
    physical: bool,
    relative: bool,
    symbolic: bool,
    suffix: Option<&'a str>,
    target_directory: Option<&'a str>,
    no_target_directory: bool,
}

/// Make links between files
pub fn ln<'a>(
    targets: &[impl AsRef<Path>],
    destination: Option<impl AsRef<Path>>,
    opts: Option<&LnOptions<'a>>,
    workdir: Option<&str>,
) -> Result<i32, Box<dyn Error>> {
    let mut cmd = Command::new("/usr/bin/env");
    cmd.arg("ln");

    if let Some(o) = opts {
        match o.backup {
            Some(BackupControl::None) => {
                cmd.arg("--backup=none");
            }
            Some(BackupControl::Off) => {
                cmd.arg("--backup=off");
            }
            Some(BackupControl::Numbered) => {
                cmd.arg("--backup=numbered");
            }
            Some(BackupControl::T) => {
                cmd.arg("--backup=t");
            }
            Some(BackupControl::Existing) => {
                cmd.arg("--backup=existing");
            }
            Some(BackupControl::Nil) => {
                cmd.arg("--backup=nil");
            }
            Some(BackupControl::Simple) => {
                cmd.arg("--backup=simple");
            }
            Some(BackupControl::Never) => {
                cmd.arg("--backup=never");
            }
            None => {
                if o.b {
                    cmd.arg("-b");
                }
            }
        }
        if o.directory {
            cmd.arg("-d");
        }
        if o.force {
            cmd.arg("-f");
        }
        if o.logical {
            cmd.arg("-L");
        }
        if o.no_dereference {
            cmd.arg("-n");
        }
        if o.physical {
            cmd.arg("-P");
        }
        if o.relative {
            cmd.arg("-r");
        }
        if o.symbolic {
            cmd.arg("-s");
        }
        if let Some(s) = o.suffix {
            cmd.args(&["-S", s]);
        }
        if let Some(d) = o.target_directory {
            cmd.args(&["-t", d]);
        }
        if o.no_target_directory {
            cmd.arg("-T");
        }
    }

    cmd.args(targets.iter().map(|t| t.as_ref()));

    if let Some(dest) = destination {
        cmd.arg(dest.as_ref());
    }

    if let Some(dir) = workdir {
        cmd.current_dir(&dir);
    }

    match cmd.status() {
        Ok(status) => Ok(status.code().unwrap_or(-1)),
        Err(err) => Err(err.into()),
    }
}

/// Simple make symlink for file
pub fn force_symlink(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<i32, Box<dyn Error>> {
    let opts = LnOptions {
        force: true,
        symbolic: true,
        ..LnOptions::default()
    };
    ln(&[src], Some(&dst), Some(&opts), None)
}

/// Call the unlink function to remove the specified file
pub fn unlink(file: impl AsRef<Path>) -> Result<i32, Box<dyn Error>> {
    let status = Command::new("/usr/bin/env")
        .arg("unlink")
        .arg(&file.as_ref())
        .status();
    match status {
        Ok(status) => Ok(status.code().unwrap_or(-1)),
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_and_unlink() {
        assert_eq!(2 + 2, 4);
        force_symlink("1.txt", "some.txt").unwrap();
        force_symlink("2.txt", "some.txt").unwrap();
        unlink("some.txt").unwrap();
    }
}
