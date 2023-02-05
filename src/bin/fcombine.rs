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
enum Output {
    Stdout,
    File( PathBuf )
}

impl TryFrom<String> for Output {

    type Error = anyhow::Error;

    fn try_from( value: String ) -> Result<Self, Self::Error> {
        let stdout_mark = "-";

        if value.is_empty() {
            bail!( "OUTPUT can not be empty" )
        } else if value == stdout_mark {
            Ok( Self::Stdout )
        } else {
            Ok( Self::File( PathBuf::from( value ) ) )
        }
    }

}


#[ derive( Debug ) ]
struct CmdOptions {
    output: Output,
    inputs: Vec<PathBuf>
}

impl CmdOptions {

    fn from_env() -> Result<Self> {

        let raw_options: Vec<String> =
            env::args().skip( 1 ).collect();

        if raw_options.len() < 2 {
            bail!( "Missing required options" )
        }

        let output = &raw_options[0];
        let inputs = &raw_options[1..];

        Ok( Self {
            output: Output::try_from( output.to_owned() )?,
            inputs: inputs
                .into_iter()
                .map( PathBuf::from )
                .collect()
        } )

    }

}



fn main() -> Result<()> {

    // Acquire command line options.

    let cmd_options =
        CmdOptions::from_env()
        .context( PROGRAM_USAGE )?;


    // Avoid accidents

    if let Output::File( ref path ) = cmd_options.output {
        if path.try_exists()? {
            bail!( "Something is already existing at \"{}\"", path.display() )
        }
    }


    // Do the IO works

    use std::fs::OpenOptions;

    let output = match cmd_options.output {
        Output::Stdout => PathBuf::from( "/dev/stdout" ),
        Output::File( path ) => path
    };

    let mut open_output =
        OpenOptions::new()
        .write( true )
        .create( true )
        .truncate( true )
        .open( &output )
        .with_context( || format!( "Failed opening \"{}\" to write", output.display() ) )?;

    for input in cmd_options.inputs {

        let mut open_input =
            OpenOptions::new()
            .read( true )
            .open( &input )
            .with_context( || format!( "Failed opening \"{}\" to read", &input.display() ) )?;

        std::io::copy( &mut open_input, &mut open_output )
            .with_context( || format!( "Error occurred copying \"{}\"", &input.display() ) )?;

    }


    Ok( () )

}
