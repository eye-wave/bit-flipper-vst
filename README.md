# Bit flipper

Very simple distortion plugin that does direct bit manipulation on audio samples, madewith [nih-plug](https://github.com/robbert-vdh/nih-plug) and [Vizia](https://github.com/vizia/vizia).
<p align="center"><img src="./assets/plugin-preview.webp"></p>

## Tested on:
- Bitwig on Pop os 22.04 🐧
- FL Studio on Windows 10 🪟

<hr>

> [!NOTE]
> I honestly thought it will sound more interesting 😭 but no.

It operates in 4 modes.
- **&** And
- **!** Not (doesn't use the bit mask)
- **|** Or
- **^** Xor

> [!WARNING]
> Flipping the most significant bits can lead to very big DC offsets.


Can i be useful? I guess so? *Well maybe if you are remaking "On sight" by Kanye.*
Flipping first fraction bits can add subtle noise to the sound.

### Download
Open actions tab and click on the most recent one that hasn't failed. Then download one for your Operating system.
> [!IMPORTANT]
> You do need to have a github account to access those files.

### License
GPL-3.0-or-later
