## Client
A ROSE Online client for the Godot Engine. 

### Goal
The goal is to create a fully functional client that supports the original
ROSE asset types out of the box.

### Setup
- Download/build godot for your platform
- Clone or download the `client/` directory

To use the original rose assets just copy them into a directory in `client/`. I
personally use `legacy/` to separate it from any new assets I create. The 
importers will automatically import any data types it recognizes (warning: might
take a while and can lock up godot until it's done).

Import Notes:
- **ZMO**: Currently it is not possible to import a ZMO without specifying a
ZMD file for it. To import a ZMO, click the file inside of godot, navigate to
import pane and select the ZMD. You can now re-import the ZMO as expected.
- **DDS**: The DDS files used in the original ROSE Client generally have issues
such as broken mipmaps.  It's recommended to convert them all to PNG and then 
let the engine handle the conversion for exported games. Some of the importing
code will look for PNGs when importing data (e.g. ZMS). Example: 
`fd -e DDS -exec convert {} {.}.png`

## Tools
A collection of tools and libraries for working with ROSE Online data
