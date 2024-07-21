use epher::argument_parser;
use epher::envinfo::ENV_INFO;

fn main() {

    let args = argument_parser::parse_args();
    args.exec();
    
    //println!("{}", ENV_INFO.config.toml());
}