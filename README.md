# WebExtension 'Edit this page' and native application launcher

Preamble TODO

Rust is a better choice for Native application than Python, principaly because Python does not install smoothly onto
Windows whereas Rust 'exe's don't require any installation and will just work.

In this repo are a Rust and a Python version of the same thing - a launcher for SeaMonkey to allow an edit this page function. You would use it in conjunction the WebExtension in this repo.

## Building the Rust application

```
cd app
cargo build --release
```

## Testing the Rust application

```
# be in the parent of the 'app' directory
python2 tester.py rust
```

If you have SeaMonkey installed, it should launch for a wikipdia page, in "edit" mode.

It would open with a second wikipedia page, but SeaMonkey doesn't do a multiple-document-interface handoff from its launch facility:

![image](https://user-images.githubusercontent.com/82182/34641477-408e6fb2-f2d3-11e7-8258-2f753ae4d86e.png)

## Testing the Python application

```
# be in the parent of the 'app' directory
python2 tester.py python
```

Same notes as for Rust.

# Installing the WebExtension

TODO

# Installing the app.

TODO

# TODO

* BlueGriffon launcher
* Bluefish launcher
* DreamWeaver?
* Word still speaks WebDAV?
* KompoZer?
* NVU?
