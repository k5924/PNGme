use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png_error::PngError;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: &EncodeArgs) -> Result<(), PngError> {
    // get basic info
    let input_file = &args.file_path;
    let chunk_type = ChunkType::from_str(args.chunk_type.as_str())?;
    let chunk_data = args.message.clone().into_bytes();
    let message_chunk = Chunk::new(chunk_type, chunk_data);
    // create Png from file path
    let mut png = Png::from_file(input_file)?;
    // add secret message chunk
    png.append_chunk(message_chunk);
    // output new file path
    if let Some(output_path) = &args.out_path {
        fs::write(output_path, png.as_bytes())?;
    }
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: &DecodeArgs) -> Result<(), PngError> {
    let input_file = &args.file_path;
    let chunk_type = args.chunk_type.as_str();
    let png = Png::from_file(input_file)?;
    let message = png.data_string_by_type(chunk_type);
    match message {
        None => { println!("no such message for chunk_type: {}.", chunk_type) }
        Some(msg) => {
            println!("secret msg for {} is: {}", chunk_type, msg)
        }
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: &RemoveArgs) -> Result<(), PngError> {
    let input_file = &args.file_path;
    let chunk_type = args.chunk_type.as_str();
    let mut png = Png::from_file(input_file)?;
    let chunk_removed = png.remove_chunk(chunk_type)?;
    fs::write(input_file, png.as_bytes())?;
    println!("remove chunk type: {}", chunk_type);
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: &PrintArgs) -> Result<(), PngError> {
    let input_file = &args.file_path;
    let mut png = Png::from_file(input_file)?;
    let v = &png.chunks;
    let num = v.len();
    println!("====================all chunk type({num})====================");
    for i in 0..num {
        print!("{}:{};", i + 1, v[i].chunk_type().to_string());
    }
    Ok(())
}
