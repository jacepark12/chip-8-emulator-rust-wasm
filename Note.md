# Resources

Handle terminal and key events : https://stackoverflow.com/questions/60130532/detect-keydown/

Test & debug rom : https://memer.eu/chip8/

# TODO
* keyboard not working
    -> Fx0A Get key not working...
* key state is not saved in wasm

# Run server
`
python3 -m http.server 9000
`

# References
https://stackoverflow.com/questions/26388861/how-can-i-include-a-module-from-another-file-from-the-same-project

https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs

wasm-pack build --debug --target web

https://github.com/rust-lang/rust/blob/master/library/core/src/clone.rs