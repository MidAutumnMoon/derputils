use std::path::PathBuf;

use anyhow::{
    bail,
    Result,
    Context
};



/// Dump contents of all <input>-s into <output> without
/// piping through shell.
#[ derive( argh::FromArgs, Debug ) ]
struct CmdOptions {

    /// a place to dump contents into
    #[ argh( positional ) ]
    output: PathBuf,

    /// from where contents are read
    #[ argh( positional ) ]
    inputs: Vec<PathBuf>

}


fn main() -> Result<()> {

    // Acquire command line options.

    let cmd_options: CmdOptions = argh::from_env();

    let output = cmd_options.output;


    // Avoid accidents

    if output.try_exists()? {
        bail!( "Something is already there at \"{}\"", output.display() )
    }


    // Do the IO works

    use std::fs::OpenOptions;

    use rustix::fs::{
        fadvise,
        Advice
    };

    let mut open_output = OpenOptions::new()
        .write( true )
        .create( true )
        .truncate( true )
        .open( &output )
        .with_context( || format!( "Failed opening \"{}\" to write", &output.display() ) )?;

    for input in cmd_options.inputs {

        let mut open_input = OpenOptions::new()
            .read( true )
            .open( &input )
            .with_context( || format!( "Failed opening \"{}\" to read", &input.display() ) )?;

        fadvise( &open_input, 0, 0, Advice::Sequential )?;

        std::io::copy( &mut open_input, &mut open_output )
            .with_context( || format!( "Error occurred copying \"{}\"", &input.display() ) )?;

    }


    Ok( () )

}
