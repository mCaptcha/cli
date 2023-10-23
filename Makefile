install.rust:
	rustup target add aarch64-linux-android
	rustup target add aarch64-unknown-linux-gnu
	rustup target add aarch64-unknown-linux-musl
	rustup target add arm-linux-androideabi
	rustup target add arm-unknown-linux-gnueabi
	rustup target add arm-unknown-linux-gnueabihf
	rustup target add arm-unknown-linux-musleabi
	rustup target add arm-unknown-linux-musleabihf
	rustup target add i586-unknown-linux-gnu
	rustup target add i586-unknown-linux-musl
	rustup target add i686-pc-windows-gnu
	rustup target add armv5te-unknown-linux-gnueabi
	rustup target add armv5te-unknown-linux-musleabi
	rustup target add armv7-linux-androideabi
	rustup target add armv7-unknown-linux-gnueabi
	rustup target add armv7-unknown-linux-gnueabihf
	rustup target add armv7-unknown-linux-musleabi
	rustup target add armv7-unknown-linux-musleabihf
	rustup target add i686-unknown-freebsd
	rustup target add i686-unknown-linux-gnu
	rustup target add i686-unknown-linux-musl
	rustup target add powerpc-unknown-linux-gnu
	rustup target add powerpc64-unknown-linux-gnu
	rustup target add x86_64-linux-android
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-unknown-freebsd
	rustup target add x86_64-unknown-linux-gnu
	rustup target add x86_64-unknown-linux-musl
	rustup target add x86_64-unknown-netbsd

define build
	rm -rf target/
	cross build --release --target $(1)
	mkdir -p build2/$(1)
	@rm target/$(1)/release/mcaptcha-cli.d
	mv target/$(1)/release/mcaptcha-cli* build2/$(1)/
	rm -rf target/
endef

build.all:
	@rm -rf build/
	@$(call build,"aarch64-linux-android")
	@$(call build,"aarch64-unknown-linux-gnu")
	@$(call build,"aarch64-unknown-linux-musl")
	@$(call build,"arm-linux-androideabi")
	@$(call build,"arm-unknown-linux-gnueabi")
	@$(call build,"arm-unknown-linux-gnueabihf")
	@$(call build,"arm-unknown-linux-musleabi")
	@$(call build,"arm-unknown-linux-musleabihf")
	@$(call build,"i586-unknown-linux-gnu")
	@$(call build,"i586-unknown-linux-musl")
	@$(call build,"i686-pc-windows-gnu")
	@$(call build,"armv5te-unknown-linux-gnueabi")
	@$(call build,"armv5te-unknown-linux-musleabi")
	@$(call build,"armv7-linux-androideabi")
	@$(call build,"armv7-unknown-linux-gnueabi")
	@$(call build,"armv7-unknown-linux-gnueabihf")
	@$(call build,"armv7-unknown-linux-musleabi")
	@$(call build,"armv7-unknown-linux-musleabihf")
	@$(call build,"i686-unknown-freebsd")
	@$(call build,"i686-unknown-linux-gnu")
	@$(call build,"i686-unknown-linux-musl")
	@$(call build,"powerpc-unknown-linux-gnu")
	@$(call build,"powerpc64-unknown-linux-gnu")
	@$(call build,"x86_64-linux-android")
	@$(call build,"x86_64-pc-windows-gnu")
	@$(call build,"x86_64-unknown-freebsd")
	@$(call build,"x86_64-unknown-linux-gnu")
	@$(call build,"x86_64-unknown-linux-musl")
	@$(call build,"x86_64-unknown-netbsd")
