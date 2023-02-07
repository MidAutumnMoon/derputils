#[ allow( dead_code ) ]
const PROGRAM_NAME: &str = "fcombine";

#[ allow( dead_code ) ]
const PROGRAM_USAGE: &str =
    "fcombine [OUTPUT or -] [INPUT]...";


use std::{
    env,
    path::PathBuf,
};

use anyhow::{
    bail,
    Result,
    Context
};



#[ derive( Debug ) ]
struct CmdOptions {
    output: PathBuf,
    inputs: Vec<PathBuf>
}

impl CmdOptions {

    fn from_env() -> Result<Self> {
        let raw_options: Vec<String> = env::args().skip( 1 ).collect();

        if raw_options.len() < 2 {
            bail!( "Missing required options" )
        }

        let output = &raw_options[0];
        let inputs = &raw_options[1..];

        Ok( Self {
            output: PathBuf::from( output ),
            inputs: inputs.iter().map( PathBuf::from ).collect()
        } )
    }

}



fn main() -> Result<()> {

    // Acquire command line options.

    let cmd_options =
        CmdOptions::from_env().context( PROGRAM_USAGE )?;

    let output = cmd_options.output;


    // Avoid accidents

    if output.try_exists()? {
        bail!( "Something is already existing at \"{}\"", output.display() )
    }


    // Do the IO works

    use std::fs::OpenOptions;

    use rustix::fs::{
        fadvise,
        Advice
    };

    let mut open_output =
        OpenOptions::new()
        .write( true )
        .create( true )
        .truncate( true )
        .open( &output )
        .with_context( || format!( "Failed opening \"{}\" to write", &output.display() ) )?;

    for input in cmd_options.inputs {

        let mut open_input =
            OpenOptions::new()
            .read( true )
            .open( &input )
            .with_context( || format!( "Failed opening \"{}\" to read", &input.display() ) )?;

        fadvise( &open_input, 0, 0, Advice::Sequential )?;

        std::io::copy( &mut open_input, &mut open_output )
            .with_context( || format!( "Error occurred copying \"{}\"", &input.display() ) )?;

    }


    Ok( () )

}
