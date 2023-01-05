use std::{
    env,
    path::PathBuf,
};

use anyhow::{
    anyhow,
    Context,
    Result,
};


/// Lookup `target` inside `path`, follow symlink recursively.
#[inline]
fn lookup( target: &str, path: &str ) -> Option<String> {

    let mut path = PathBuf::from( path );

    path.push( target );

    match path.canonicalize() {
        Ok( full_path ) => Some( full_path.to_string_lossy().to_string() ),
        Err( _ ) => None
    }

}


fn main() -> Result<()> {

    let self_name =
        env::current_exe()
        .context( "Failed getting path of current executable." )?;

    let self_name =
        self_name.file_name()
        .ok_or_else( || anyhow!( "Failed getting name of current executable." ) )?
        .to_string_lossy();


    let target_program =
        env::args()
        .nth( 1 )
        .ok_or_else( || anyhow!( "{self_name}: [program name]" ) )?;


    let env_path =
        env::var( "PATH" ).context( "Failed reading $PATH" )?;

    for path in env_path.rsplit( ':' ) {
        let lookup_result =
            lookup( &target_program, path );

        if let Some( full_path ) = lookup_result {
            println!( "{}", full_path );
            return Ok( () )
        }
    }


    Err( anyhow!( "Program \"{target_program}\" not found while iterating $PATH" ) )

}
