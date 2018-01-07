Warning: This is an alpha quality product that is still under development. It is not ready for production
or end user use yet.

# WebExtension 'Edit this page' and native application launcher

Preamble TODO

Rust is a better choice for Native application than Python, principaly because Python does not install
smoothly onto Windows whereas Rust 'exe's don't require any installation and will just work.

In this repo are a Rust and a Python version of the same thing - a launcher for SeaMonkey to allow an
edit this page function. You would use it in conjunction the WebExtension in this repo.

# Building/Deploying

## Building the Rust application

```
cd app
cargo build --release
```

## Testing the implementations

There's a tester script that confirms that the Rust production implementation conforms
to the Python prototype.  It's not a unit test tech, just a simple script that controls
a second process, as Firefox would.

### Testing the Rust application

You will have built the Rust executable first with cargo (as above).

```
# be in the parent of the 'app' directory
python2 tester.py rust
```

If you have SeaMonkey installed, it should launch for a wikipdia page, in "edit" mode.

It would open with a second wikipedia page, but SeaMonkey doesn't do a multiple-document-interface
handoff from its launch facility:

![image](https://user-images.githubusercontent.com/82182/34641477-408e6fb2-f2d3-11e7-8258-2f753ae4d86e.png)

### Testing the Python application

You will have Python2 installed, and done `pip2 install termcolor` for an extra package.

```
# be in the parent of the 'app' directory
python2 tester.py python
```

Same notes as for Rust.

# Installation

## Installing the native app

The extension does nothing without the native app. This **tiny** app receives "edit this page" commands
from Firefox/Chrome and then launches SeaMonkey to edit the page.

That needs to be installed on your desktop outside the browser. You need to install the one for your
platform (MacOS, Linux, Windows).

There's a `manifest.json` file that needs to be deployed to within Firefox and Chrome. Here is
a page at [Mozilla Dev Network: Native manifests location](https://developer.mozilla.org/en-US/Add-ons/WebExtensions/Native_manifests#Manifest_location).
Inside that manifest, there's a line for the full-qualified path to the `edit_this_page` executable.
You'll need to change that. In the future I will make scripts to do the install for you.

As ever you should be aware of what you're installing on your machine. The executuable, regardless of
platform is compiled from this source file [https://github.com/paul-hammant/EditThisPage_WebExtension/blob/master/app/src/main.rs](https://github.com/paul-hammant/EditThisPage_WebExtension/blob/master/app/src/main.rs). You don't have to be super advanced with programming
languages to read the source and see that it is 'ok' to install. If only we had a way of utilizing
Mozilla's Continuous Integration to create and trusted 'exe' for each platform - **at install time**.

### Footprint

The app seems to take less that 1MB of memory and one thread while running (it runs perpetually presently).
The Mac's activity monitor is reporting that there's no disk, no network and (effectively) no CPU being
used by it.

## Installing the WebExtension

TODO

Right now, only developers are going to be installing the web extension. I myself do this from the Mac command
line:

```
cd web-extension/
web-ext run --verbose --firefox=/Applications/FirefoxDeveloperEdition.app/Contents/MacOS/firefox-bin
```

# Future features.

* BlueGriffon launcher
* Bluefish launcher
* DreamWeaver?
* Word still speaks WebDAV?
* KompoZer?
* NVU?

# Credits

Alexandr Lyashenko quickly wrote the Rust version for me via the UpWork platform (based on the
Python2 version I had written). [His link there](https://www.upwork.com/freelancers/~01af9df894ec5fb364),
and granted copyright to me. Credit where it's due though :)
