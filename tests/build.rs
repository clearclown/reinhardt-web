fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Compile proto files for gRPC integration tests
	tonic_build::configure()
		.build_server(true)
		.build_client(true)
		.compile(
			&[
				"proto/common.proto",
				"proto/user.proto",
				"proto/user_events.proto",
			],
			&["proto"],
		)?;

	Ok(())
}
