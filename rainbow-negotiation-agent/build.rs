use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap_or("out".to_string()));
    let descriptor_path = out_dir.join("negotiation_descriptor.bin");
    tonic_prost_build::configure()
        .compile_well_known_types(false)
        .file_descriptor_set_path(descriptor_path)
        .compile_protos(
            &[
                "proto/models.proto",
                "proto/negotiation-process.proto",
                "proto/negotiation-message.proto",
                "proto/offer.proto",
                "proto/agreement.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
