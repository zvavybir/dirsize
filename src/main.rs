use std::{
    fs::{metadata, read_dir},
    io,
    path::PathBuf,
};

use clap::{crate_authors, crate_name, crate_version, App, Arg};

fn size_recursive(path: PathBuf) -> Result<usize, io::Error>
{
    let mut i = 0;
    let mut values = vec![];

    for dir in read_dir(&path)?
    {
        let path = dir?.path();
        if metadata(&path).map(|d| d.is_dir()).unwrap_or(false)
        {
            values.push(size_recursive(path)?);
        }
        i += 1;
    }

    values.push(i);

    values.sort_unstable_by(|a, b| b.cmp(a));

    Ok(values[0])
}

fn main()
{
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Calculates directory sizes")
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .help("Traverses directories recursively"),
        )
        .arg(
            Arg::with_name("directories")
                .required(true)
                .multiple(true)
                .help("Directories to search through"),
        )
        .get_matches();

    let get_size = if matches.is_present("recursive")
    {
        size_recursive
    }
    else
    {
        |path| Ok(read_dir(path)?.count())
    };

    let mut all_vals = matches
        .values_of_os("directories")
        .unwrap()
        .map(PathBuf::from)
        .map(get_size)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    all_vals.sort_unstable_by(|a, b| b.cmp(a));

    println!("{}", all_vals[0]);
}
