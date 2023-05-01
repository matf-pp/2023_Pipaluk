# Pipaluk

[![Codacy Badge](https://app.codacy.com/project/badge/Grade/92df916caab042f7bbc49bfa17fe51a8)](https://app.codacy.com/gh/matf-pp/2023_Pipaluk/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)

A game about a cat named Pipaluk trying to find a way out of a long dead city of hopeless robots.

<p float="left" align="middle">
  <img src="screenshots/pipaluk.png" width="32%" />
  <img src="screenshots/robots.png" width="32%" />
  <img src="screenshots/pipaluk_animated.gif" width="32%" />
</p>

Thread carefully along the streets of this strange city and delve deep into the underground corridors underneath it, while avoiding and running from the not-so-friendly residents.

<p float="left" align="middle">
  <img src="screenshots/map_sewers.png" width="49%" />
  <img src="screenshots/map_labyrinth.png" width="49%" />
</p>

## Run Pipaluk 

### Linux (Ubuntu)

You will need SDL2:
```
sudo apt-get install libsdl2-dev libsdl2-mixer-dev libsdl2-image-dev libsdl2-ttf-dev
```

Download the latest `pipaluk_linux.zip` from releases, and run the `pipaluk` executable:
```
./pipaluk
```

If you lack permissions to execute, run:
```
sudo chmod u+x pipaluk
```

### Windows

Download the latest `pipaluk_windows.zip` from releases and extract it.

Run `pipaluk.exe`

## Build Pipaluk

In order to build the project, you will need to [install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (both Linux and Windows).

### Linux (Ubuntu)

You will need SDL2:
```
sudo apt-get install libsdl2-dev libsdl2-mixer-dev libsdl2-image-dev libsdl2-ttf-dev
```

Clone the project and navigate to it via terminal:
```
git clone https://github.com/matf-pp/2023_Pipaluk.git
cd 2023_Pipaluk
```

The following command will install any required libraries, build, and finally run the game:
```
cargo run
```

### Windows

Download VC versions of [SDL2](https://github.com/libsdl-org/SDL/releases/), [SDL2-image](https://github.com/libsdl-org/SDL_image/releases/), [SDL2-ttf](https://github.com/libsdl-org/SDL_ttf/releases/), and [SDL2-mixer](https://github.com/libsdl-org/SDL_mixer/releases/).

Put the `.lib` files inside the project folder; build and run using cargo:
```
cargo run
```


## Credits

The game was developed with love by Marijana Čupović ([Marijameme](https://github.com/Marijameme)), Vuk Amidžić ([vukamidzic](https://github.com/vukamidzic)), and Daniil Grbić ([daniilgrbic](https://github.com/daniilgrbic)).

Big thanks to Lara Ritan for the awesome soundtrack.

Maps by Marijana Čupović.

Art by Daniil Grbić.
