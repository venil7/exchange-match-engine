// extern crate prost_build;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let proto_dir = "../proto";
//     let proto_file = format!("{dir}/exchange.proto", dir=proto_dir);

//     prost_build::Config::new()
//         .out_dir("./src")
//         .type_attribute("OrderRequest", "#[derive(serde::Deserialize, serde::Serialize)]")
//         .compile_protos(&[proto_file], &[proto_dir])?;

//         Ok(())
// }
