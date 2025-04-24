extern crate base85;
use base85::{decode, encode};

pub mod diropql;
pub mod compressor;


fn main() {
    let message = "Kuromi is such a cute character. She is so me! Kuromi is such a cute character. She is so me! Kuromi is such a cute character. She is so me! Kuromi is such a cute character. She is so me! Kuromi is such a cute character.".to_string();

    let drpql_program = diropql::zip::write_diropql(&message);
    println!("Encoded Diropql program: {}", drpql_program);

    let read_program = diropql::zip::read_diropql(&drpql_program);
    println!("Decoded Diropql program: {}", read_program);

    let drpqlz_program = diropql::zip::write_diropqlz(&message);
    println!("Encoded Diropqlz program: {}", drpqlz_program);

    let readz_program = diropql::zip::read_diropqlz(&drpqlz_program);
    println!("Decoded Diropqlz program: {}", readz_program);
}
