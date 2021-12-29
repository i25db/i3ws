# i3ws
The purpose of this project is mostly to learn rust and along the way make a tool to manage my workspaces. I've only been on Linux for a month or two now and I've already found myself limited by the default 10 workspaces i3 has. i3ws can be used in a situation like this to group multiple workspaces into one workspace
## Features
- Navigate through the 10 main workspaces (ex. `i3ws -g main 0`)
- Navigate through the sub workspaces (ex. `i3ws -g sub 0`)
- Create a new workspace from preset (ex. `i3ws -n code`)
- **And more incoming** <br/> <br/>
When properly setup with shortcuts in your i3 config you can easily access 25+ workspaces
## Dependencies
- **i3wm** or **i3-gaps** naturally
- **jq** for parsing i3-msg output
## Installation
At the moment there is not installation script. So you'll just have to build the project:<br/>
`git clone https://github.com/i25db/i3ws.git && cd i3ws && cargo build --release`<br/>
Copy the binary `target/release/i3ws` to some location in your path
## Commands
- **i3ws [-g | go] [main | sub] [index]** <br/>
This command will navigate to either the first sub workspace of an existing main workspace, or a sub workspace of the currently focused main workspace
- **i3ws [-n | new] [plain | code | game]** <br/>
If the current main workspace (including all of its subworkspaces) has no windows open this will load a preset into the main workspace
- **i3ws [-s | swap] [main | sub] [index]** <br/>
Swaps the focused main or sub workspace with the given main or sub index
- **i3ws -d** <br/>
Goes to the default workspace
## Configuration
No support for configuration. Definitely planned though
## TODO
- [x] Manage main workspaces
- [x] Manage sub workspaces
- [x] Create workspace preset
- [ ] Start applications when creating a preset
- [ ] Move and swap main/sub workspaces
- [x] Make it configurable
- [ ] Output something nice that things like polybar, eww, etc... can use
- [ ] Create an install script
