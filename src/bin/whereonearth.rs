const PROGRAM_NAME: &str = "whereonearth";

use std::env;
use std::path::PathBuf;

use anyhow::{
    anyhow,
    Context,
    Result,
};


fn main() -> Result<()> {

    let target_program =
        env::args()
        .nth( 1 )
        .ok_or_else( || anyhow!( "{PROGRAM_NAME}: [program name]" ) )?;

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


    Err( anyhow!( "Program \"{target_program}\" not found while iterating $PATH" ) )

}
