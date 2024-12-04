# RealTime audio DSP
This project provide a multichannel DSP, with an web server dedicated to remote control
It is design for Alsa & Linux based systems.

## Current implementations
* 2 Channels 24bits 48kHz inputs and outputs using HifiBerry SPDIF I/O
*  32 bits integer processing
* Channel processing : Trim, 6 bands EQ, peak & RMS Metering

* Remote control is currently under migration from nodejs, to be hosted directly in the app, most of the remote control isn't working until full migration
* Control of all DSP parameters from web page UI

## TODO
* Bypassing of the nodejs server (src/control/web_server/server.js) to limit the use of Javascript
* Implementation of the 6bands full EQ is on hold as its UI is to be built
* Dynamics 

### About
The DSP contains unused legacy filters (LPF, HPF and band filter); Replaced by the Full EQ.
I/O and DSPs handling are designed to handle multichannel PCM Devices, up to what your CPU can handle...

#### Crates & libs 
This repo gratefully rely on this great crate for  :
Alsa lib wrapping for Rust : https://github.com/diwic/alsa-rs.git