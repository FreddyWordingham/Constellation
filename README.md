# Constellation

<div align="center">
    <img src="./images/logo.svg" alt="Galaxy simulation" width=200>
</div>

N-body simulation.

![Rendered simulation](./images/galaxy.gif)

## Quickstart

Clone the repository and change into the directory:

```shell
git clone https://github.com/FreddyWordingham/Constellation
cd Constellation
```

Compile the code:

```shell
cargo run --release -- --radius 1e6 --res 512 --grav-strength 1.0e2 --smoothing-length 1e3 --num-stars 40000 --cmap 000000 000022 000077 AA00AA AA0000 FFFF00 FFFFFF
```

![Terminal simulation](./images/screenshot.png)
