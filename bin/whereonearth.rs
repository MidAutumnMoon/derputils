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
fn lookup( target: &str, path: &PathBuf ) -> Option<String> {

    let mut path = path.to_owned();

    path.push( target );

    match path.canonicalize() {
        Ok( full_path ) => Some( full_path.to_string_lossy().into_owned() ),
        Err( _ ) => None
    }

}


fn main() -> Result<()> {

    let self_name =
        env::current_exe()
        .context( "Failed getting path of current executable." )?;

    let self_name =
        self_name.file_name()
        .ok_or( anyhow!( "Failed getting name of current executable." ) )?
        .to_string_lossy();


    let target_program =
        env::args()
        .nth( 1 )
        .ok_or( anyhow!( "{self_name}: [program name]" ) )?;


    let paths =
        env::var( "PATH" )
        .context( "Failed reading $PATH" )?;

    let paths =
        paths.rsplit( ":" );

    for path in paths {
        let lookup_result =
            lookup( &target_program, &PathBuf::from( path ) );
        if let Some( full_path ) = lookup_result {
            println!( "{}", full_path );
            return Ok( () )
        }
    }


    Err( anyhow!( "Program \"{target_program}\" not found while iterating $PATH" ) )

}
