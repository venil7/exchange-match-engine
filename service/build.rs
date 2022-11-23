fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "../proto";
    let proto_file = format!("{dir}/exchange.proto", dir=proto_dir);

    tonic_build::configure()
        .build_server(true)
        .out_dir("src/")
        .type_attribute("OrderRequest", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile(&[proto_file], &[proto_dir])?;
    Ok(())
}
