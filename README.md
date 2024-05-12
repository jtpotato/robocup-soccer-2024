# ferret

<img src="assets/image.png" width="200" alt="AI art of a ferret kicking a soccer ball">

Program for a robot in Robocup Soccer (RCJA 2024 Rules), using [pixix4/ev3dev-lang-rust](https://github.com/pixix4/ev3dev-lang-rust)

## Features

- [x] Follows Ball
- [x] Follows initial compass direction
- [x] Different control regimes depending on whether robot has/does not have ball.

## Why did you... ⁉️

**... use Rust? 🦀**

- It's fast
- It's cool
- The [US Government](https://arc.net/l/quote/xfwrzrvl) says you should use Rust 🦅
- I would use a garbage collector if I could, but performance 😔
- Actual type system

**... use Linux? 🐧**

- How else were we supposed to get Rust 🦀 working?
- ev3dev has done the hard work for me
- ~~so that I can say I use Arch, btw~~ for legal reasons, that was a joke, ev3dev is a Debian distro
- I like penguins

**... not just use Scratch?**

> um actually, it's LEGO's Scratch-like language called [insert name here, I have no idea what its called] 🤓☝️

- If you have ever used a programming language in a text editor with actual keybindings you will understand how painful moving blocks around can be.

- The IR seeker sensor requires a very specific AC mode in order to not detect sunlight and only detect the infrared soccer ball. This mode no longer exists as EV3 Classroom exposes the IR seeker through the "ultrasonic sensor" block, which obviously doesn't have AC/DC modes (or the other super-useful ones). [See: NXC HiTechnic API](https://bricxcc.sourceforge.net/nbc/nxcdoc/nxcapi/group___hi_technic_a_p_i.html) _This offers a clear advantage compared to teams that use LEGO's default tool_

- ev3dev retains most of these features, allowing libraries to make use of them.
