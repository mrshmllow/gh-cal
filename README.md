# View your github contribution calander in the terminal

![image](https://user-images.githubusercontent.com/40532058/166221848-c6db8603-9bbd-4328-b0f8-21e25bd1aac4.png)

![image](https://user-images.githubusercontent.com/40532058/166221881-a15652e4-3d92-4a4a-bed2-cefeeb735883.png)

Now in rust!

```
gh-cal 0.1.0
marshmallow
View your github contribution calander in unicode

USAGE:
    gh-cal [OPTIONS] [USERNAME]

ARGS:
    <USERNAME>    A valid Github username. Tries to guess

OPTIONS:
    -h, --help         Print help information
    -n, --no-colour    Disable colour. Respects NO_COLOR by default
    -r, --raw          See raw levels
    -V, --version      Print version information
```

# Install

```sh
git clone https://github.com/mrshmllow/gh-cal
cd gh-cal
tar -xf gh-cal.tar.xz
sudo cp gh-cal /usr/bin
sudo chmod +x /usr/bin/gh-cal
```

> AUR (Needs approval from the Author)

```sh
yay -S gh-cal
```

# Build

> You must have cargo installed.

```sh
cargo install gh-cal
```
