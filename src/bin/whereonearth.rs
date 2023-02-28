use std::env;

use std::path::{
    Path,
    PathBuf,
};

use anyhow::{
    bail,
    Context,
    Result,
};


/// Find executable "name" in $PATH, following the symlink
/// to reveal its true location.
#[ derive( argh::FromArgs, Debug ) ]
struct CmdOptions {
    /// find the executable with this name
    #[ argh( positional ) ]
    name: String
}


fn is_executable( path: &Path ) -> Result<bool> {
    use std::os::unix::fs::PermissionsExt;
    let metadata = path.metadata()?;
    let permission = metadata.permissions();
    Ok(
        metadata.is_file()
        && permission.mode() & 0o111 != 0
    )
}


fn main() -> Result<()> {

    let cmd_opts: CmdOptions = argh::from_env();

    let envvar_path =
        env::var( "PATH" ).context( "Failed reading $PATH" )?;


    for location in envvar_path.rsplit( ':' ) {

        let mut location = PathBuf::from( location );

        location.push( &cmd_opts.name );

        let full_path = match location.canonicalize() {
            Ok( p ) => p,
            Err(_) => continue,
        };

        match is_executable( &full_path ) {
            Ok( true ) => {
                println!( "{}", &full_path.display() );
                return Ok(())
            },
            Ok( false ) => continue,
            Err(_) => continue,
        }

    }


    bail!( "Program \"{}\" not found while iterating $PATH", &cmd_opts.name )

}
