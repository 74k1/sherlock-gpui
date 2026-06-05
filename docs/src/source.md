### <ins>From Source</ins>

To build Sherlock from source, follow these steps. 

Make sure to have the following dependencies installed:

<details>
<summary><strong>Dependencies</strong></summary>

1. `rust` - [How to install rust](https://www.rust-lang.org/tools/install)
2. `git` - [How to install git](https://github.com/git-guides/install-git)
3. `gtk-4-layer-shell` - [GTK4 Layer Shell](https://github.com/wmww/gtk4-layer-shell)
4. `dbus` - (Used to get currently playing song)

</details>

<details>
<summary><strong>Build Steps:</strong></summary>

1. **Clone the repository**:

   ```bash
   git clone https://github.com/skxxtz/sherlock.git
   cd sherlock
   ```

2. **Build the project using the following command**:

   ```bash
   cargo build --release
   ```

3. **Install the binary**:
   After the build completes, install the binary to your system:

   ```bash
   sudo cp target/release/sherlock /usr/local/bin/
   ```

4. **(Recommended) Remove the build directory**:
   You can optionally remove the source code directory

   ```bash
   rm -rf /path/to/sherlock
   ```

</details>