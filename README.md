# Zigurate Lang

<div align="left">

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
<img src="https://img.shields.io/badge/Edition-2021-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust Edition" />

<img src="https://img.shields.io/badge/Status-Beta%206-blue?style=for-the-badge" alt="Status Beta 6" />
<img src="https://img.shields.io/github/v/release/G4brielXavier/ZigurateDSL?style=for-the-badge&color=brightgreen" alt="Release" />

<img src="https://img.shields.io/github/license/G4brielXavier/ZigurateDSL?style=for-the-badge&color=yellow" alt="License" />
<img src="https://img.shields.io/github/issues/G4brielXavier/ZigurateDSL?style=for-the-badge&logo=github" alt="Issues" />

</div>

> Created specifically for my personal worldbuilding use. If you have patient to use it for your world, be free.

> Nowadays, Zigurate UR1 is in deep development, so, some features still not arrive. That's why it is in beta.

*A domain-specific language for game designers, writters and worldbuilders, focused on query and local database. Designed in **Rust**.*


![banner](./imgs/ZigurateBanner.png)


```js
silim("Hello World!")
```


## What's Zigurate?

I designed it to speed-up my large worldbuilding search, I have a lot of characters, artifacts, items and others. With Zigurate, I can organize my world and search for what I want to find.

[View documentation](https://ziguratedsl.vercel.app)

## Download

### Github Releases (.zip)

1. Go to [Releases](https://github.com/G4brielXavier/ZigurateLang/Releases)
2. Choice the version and install the *enki_installer-URX.zip*
3. After, extract the `.zip`.
4. Open the folder and execute `install.bat`
5. Close the terminal that will be open.
6. Open a new terminal and execute: `enki --version`

### Cargo

1. Open your terminal and execute:
```ps
cargo install zigurate
```
2. After install, open a new terminal and use:
```ps
enki --version
```



## Roadmap

| Milestone | Status |
|---|---|
| New *AnkiFunction*: `anki.:muria()` for timelines creations | ❌ |
| New Comparison system: `gina` (like `match` in rust) specifically to compare `GishOptions` | ❌ |
| New CLI command: `enki galam --file` to convert/export `lunig`'s to `json` that can be used in APIs. | ✅ |
| Official documentation with VitePress | ✅ |


## LICENSE

**MIT Licence -** [Read here](./LICENSE)