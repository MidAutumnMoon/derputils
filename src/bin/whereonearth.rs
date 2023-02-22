use std::env;
use std::path::PathBuf;

use anyhow::{
    bail,
    Context,
    Result,
};


/// Find executable <name> in $PATH, with following the symlink
/// to reveal its true location.
#[ derive( argh::FromArgs, Debug ) ]
struct CmdOptions {
    /// find the executable with this name
    #[ argh( positional ) ]
    name: String
}


fn main() -> Result<()> {

    let cmd_options: CmdOptions = argh::from_env();

    let envvar_path =
        env::var( "PATH" ).context( "Failed reading $PATH" )?;

    for location in envvar_path.rsplit( ':' ) {

        let mut location = PathBuf::from( location );

        location.push( &cmd_options.name );

        if let Ok( full_path ) = location.canonicalize() {
            println!( "{}", full_path.display() );
            return Ok( () )
        }

    }


    bail!( "Program \"{}\" not found while iterating $PATH", &cmd_options.name )

}
