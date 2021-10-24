mod eosio_bin;
mod eosio_types;
mod ship_types;

extern crate clap;

use clap::Parser;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use std::io::{Error, ErrorKind, Result};

use eosio_bin::*;
use eosio_types::*;
use ship_types::*;

#[derive(Parser, Debug)]
#[clap(name = "hello-rust")]
struct Options {
    /// Path to chain_state_history.log
    file: String,

    /// Skip decompression. Assumes nodeos was patched to not compress deltas.
    #[clap(short, long)]
    no_decompress: bool,
}

fn show_tables(src: &mut &[u8]) -> Result<()> {
    let num_table_deltas = Varuint32::eosio_deserialize(src)?.value as usize;
    println!("num_table_deltas: {}", num_table_deltas);

    for _ in 0..num_table_deltas {
        let delta = TableDelta::eosio_deserialize(src)?;
        let delta_v0 = match delta {
            TableDelta::TableDeltaV0(d) => Ok(d),
            _ => Err(Error::new(ErrorKind::Other, "wrong nodeos version")),
        }?;
        println!(
            "table: {:30} rows:{:10}",
            std::str::from_utf8(delta_v0.name.value)
                .map_err(|_| Error::new(ErrorKind::Other, "utf8 error"))?,
            delta_v0.rows.len()
        );
    }
    Ok(())
}

fn main() {
    let options = Options::parse();
    let result = (|| {
        let bin = std::fs::read(options.file)?;
        let mut pos = &bin[..];
        let _ = StateHistoryLogHeader::eosio_deserialize(&mut pos)?;
        let compressed_size = u32::eosio_deserialize(&mut pos)? as usize;
        let decompressed: Vec<u8>;
        let mut pos = {
            if options.no_decompress {
                pos
            } else {
                println!("decompressing...");
                decompressed = decompress_to_vec_zlib(&pos[..compressed_size])
                    .map_err(|_| Error::new(ErrorKind::Other, "zlib error"))?;
                println!("decompressed");
                &decompressed[..]
            }
        };
        show_tables(&mut pos)
    })();
    if let Err(err) = result {
        println!("{}", err);
    }
}
