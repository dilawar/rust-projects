name:=Pixel_API_33
image:=system-images;android-33-ext5;google_apis_playstore;x86_64

all:
	yes | sdkmanager --licenses 
	dx serve --platform android

setup_emulator: create_emulator
	emulator -list-avds
	emulator -avd $(name) -netdelay none -netspeed full

create_emulator:
	@echo "Downloading image..."
	# sdkmanager --list 
	yes | sdkmanager --install "$(image)"
	yes | sdkmanager --licenses	
	echo no | avdmanager create avd --force --name $(name) --abi x86_64 --package "$(image)"
