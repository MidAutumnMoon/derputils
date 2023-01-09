// fcombine [destination] [origin] ...
//
// Combine contents of many files into the destination
// file without using shell pipelines.


const PROGRAM_NAME: &str = "fcombine";


use anyhow::{
    bail,
    Result,
    Context
};


fn main() -> Result<()> {

    // Acquire command line options.

    let cmd_options =
        std::env::args()
        .skip( 1 )
        .collect::<Vec<String>>();

    if cmd_options.len() < 2 {
        bail!( "{PROGRAM_NAME}: [destination] [origin] ..." )
    }

    let destination = &cmd_options[0];

    let origins = &cmd_options[1..];


    // Avoid accidents

    use std::path::Path;

    if Path::new( destination ).exists() {
        bail!( "Something is already here at \"{destination}\"" )
    }


    // Do the IO works

    use std::fs::{
        File,
        OpenOptions,
    };

    let mut opened_destination =
        OpenOptions::new()
        .write( true )
        .create( true )
        .truncate( true )
        .open( destination )
        .with_context( || format!( "Failed opening \"{destination}\" to write" ) )?;

    for origin in origins {

        let mut opened_origin =
            File::open( origin )
            .with_context( || format!( "Failed opening \"{origin}\" to read" ) )?;

        std::io::copy( &mut opened_origin, &mut opened_destination )
            .with_context( || format!( "Error occurred copying \"{origin}\"" ) )?;

    }


    Ok( () )

}
