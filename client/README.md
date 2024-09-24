# Client

This client is built upon the Tauri framework v2 [docs here](https://v2.tauri.app/start/)

## Development

### Requirements

From the [tauri docs](https://v2.tauri.app/start/prerequisites/) we need to install 
#### Debian

```
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

#### Arch

```
sudo pacman -Syu
sudo pacman -S --needed \
  webkit2gtk-4.1 \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  libappindicator-gtk3 \
  librsvg
```

#### Fedora
```
sudo dnf check-update
sudo dnf install webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel
sudo dnf group install "C Development Tools and Libraries"
```

On this project I use **bun** which can be install with the following command
``` 
  curl -fsSL https://bun.sh/install | bash # for macOS, Linux, and WSL
```

When you first git clone this project you need to install the nodejs dependencies
```
  bun install
```

### Start the dev client

Now to start the development version of the client
```
  bunx tauri dev
```
## Troubleshooting
### Pop-shell window exception

For the users that are using [pop-shell](https://github.com/pop-os/shell#installation), you can add the client window(s) as an exception to the auto tilling feature. This will make the meme windows keep its default size and appear on the screen as it should be. To do so : 
  1. click on the pop-shell app indicator in your status bar
  2. click on “Floating window exceptions”
  3. click on “Select”
  4. wait for a meme window to appear and click on it
  5. click on “This app's windows”.
