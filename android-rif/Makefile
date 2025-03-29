run:
	flutter run

bootstrap: check
	@echo We assume that you are already inside flutter project
	flutter pub add rinf 
	cargo binstall rinf
	rinf template

check:
	rustc --version
	flutter doctor
