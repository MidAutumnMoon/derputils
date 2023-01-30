const PROGRAM_NAME: &str = "whereonearth";

use std::env;
use std::path::PathBuf;

use anyhow::{
    bail,
    Context,
    Result,
};


fn main() -> Result<()> {

    let target_program =
        env::args()
        .nth( 1 )
        .with_context( || format!( "{PROGRAM_NAME}: [program name]" ) )?;

    let env_path =
        env::var( "PATH" ).context( "Failed reading $PATH" )?;


    for path in env_path.rsplit( ':' ) {

        let mut path = PathBuf::from( path );

        path.push( &target_program );

        if let Ok( full_path ) = path.canonicalize() {
            println!( "{}", full_path.to_string_lossy() );
            return Ok( () )
        }

    }


    bail!( "Program \"{target_program}\" not found while iterating $PATH" )

}
