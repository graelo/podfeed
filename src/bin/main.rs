use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use clap::{CommandFactory, Parser};
use clap_complete::generate;

use podsync::{
    config::{self, Config},
    Result,
};

fn main() -> Result<()> {
    let config = Config::parse();

    match config.command {
        config::Command::Generate { data_dir, base_url } => {
            let base_url = Path::new(&base_url);
            async_std::task::block_on(run(&data_dir, base_url))?;
        }
        config::Command::GenerateCompletion { shell } => {
            let mut app = Config::command();
            let name = app.get_name().to_string();
            generate(shell, &mut app, name, &mut std::io::stdout());
        }
    }

    Ok(())
}

async fn run(data_dir: &Path, base_url: &Path) -> Result<()> {
    let directories = podsync::convert::available_directories(data_dir).await?;

    for dirpath in &directories {
        println!("- {}", dirpath.to_string_lossy());
        let rss_content = podsync::convert::process(data_dir, dirpath, base_url).await?;
        let rss_filepath = append_ext("xml", dirpath);
        async_std::fs::write(rss_filepath, rss_content).await?;
        // let rss_filepath = data_dir.with_file_name()
        // println!("{}", rendered_rss);
    }
    Ok(())
}

/// Returns a path with a new dotted extension component appended to the end.
/// Note: does not check if the path is a file or directory; you should do that.
/// # Example
/// ```
/// use pathext::append_ext;
/// use std::path::PathBuf;
/// let path = PathBuf::from("foo/bar/baz.txt");
/// if !path.is_dir() {
///    assert_eq!(append_ext("app", path), PathBuf::from("foo/bar/baz.txt.app"));
/// }
/// ```
///
fn append_ext(ext: impl AsRef<OsStr>, path: &Path) -> PathBuf {
    let mut os_string: OsString = path.into();
    os_string.push(".");
    os_string.push(ext.as_ref());
    os_string.into()
}
