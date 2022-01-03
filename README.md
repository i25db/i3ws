<a name="desc"></a>
# i3ws Introduction
The purpose of this project is mostly to learn rust and along the way make a tool to manage my workspaces. I've only been on Linux for a month or two now and I've already found myself limited by the default 10 workspaces i3 has. i3ws can be used in a situation like this to group multiple workspaces into one workspace
## Table of Contents
- [Introduction](#desc)
- [Features](#features)
- [Dependencies](#deps)
- [Installation](#installation)
- [Commands](#commands)
  - [i3ws go](#commands-go)
  - [i3ws new](#commands-new)
  - [i3ws swap](#commands-swap)
  - [i3ws info](#commands-info)
  - [i3ws default](#commands-default)
- [Configuration](#conf)
  - [i3 Configuration](#i3-conf)
  - [sxhkd Configuration](#sxhkd-conf)
  - [i3ws Configuration](#i3ws-conf)
- [Todo](#todo)
<a name="features"></a>
## Features
- Navigate through the main workspaces (ex. `i3ws go main 1`)
- Navigate through the sub workspaces (ex. `i3ws go sub 1`)
- Create a new workspace from preset (ex. `i3ws new code`)
- **And more incoming** <br/> <br/>
When properly setup with shortcuts in your i3 config you can easily access 25+ workspaces
<a name="deps"></a>
## Dependencies
- **i3wm** or **i3-gaps** naturally
- **jq** for parsing i3-msg output
<a name="isntallation"></a>
## Installation
At the moment there is a basic installation script. Just run `./install.sh`
<a name="commands"></a>
## Commands
<a name="commands-go"></a>
- **`i3ws go [main | sub] [index]`** <br/>
This command will navigate to either the first sub workspace of an existing main workspace, or a sub workspace of the currently focused main workspace
<a name="commands-new"></a>
- **`i3ws new [your presets here]`** <br/>
If the current main workspace (including all of its subworkspaces) has no windows open this will load a preset into the main workspace
<a name="commands-swap"></a>
- **`i3ws swap [main | sub] -d [index]`** <br/>
Swaps the focused main or sub workspace with the given main or sub index
<a name="commands-info"></a>
- **`i3ws info [current | all_mains | all_subs]`** <br/>
Prints the name of the focused main workspace, a list of all the main workspace names or a list of
all the sub workspace names. See [i3ws configuration](#i3ws-conf). 
<a name="commands-default"></a>
- **`i3ws default`** <br/>
Goes to the default workspace. Run this command at startup so that your first workspace doesn't get
lost
<a name="conf"></a>
## Configuration
Here you will find some example configurations for i3, sxhkd and i3ws
<a name="i3-conf"></a>
## i3 Configuration
Really all i3 needs to do is run `exec --no-startup-id i3ws default`. See [sxhkd configuration](#sxhkd-conf) for shortcut setups
<a name="sxhkd-conf"></a>
## sxhkd Configuration
```
super + {0-9}
  i3ws go sub {0-9}
  
super + shift + {0-0}
  i3ws swap {0-9}
  
super + w; {0-9}
  i3ws go main {0-9}
  
super + shift + w; {0-9}
  i3ws swap {0-9} -d main
  
super + n; {1}
  i3ws new {code}
```
<a name="i3ws-conf"></a>
## i3ws Configuration
The default configuration file (located at .config/i3ws/i3ws.toml): <br/>
```ini
# Which workspace preset to start on. Required
default_type = "plain"

# When swapping sub workspaces whether to move to the 
# destination workspace
# default: false
swap_on_sub = false

# When swapping main workspaces whether to move to the
# destination workspace
# default: true
swap_on_main = true

# This setting will be removed shortly. Keep true to
# enable workspace swapping
# default: true
swap_on_default_only = true


[[types]]

# The name of this workspace preset. Required
name = "plain"

# The name that will be printed for the main workspace
# formats:
#    {index} - The main index of the workspace
# default: "{index}"
display_name = ""

# The name that will be printed for the main workspace
# if it is focused
# formats:
#   {index} - The main index of the workspace
# default: "{index"
display_name_focused = ""

# The name that will be printed for the sub workspace
# formats:
#    {index} - The sub index of the workspace
# default: "{index}"
sub_display_name = ""

# The name that will be printed for the sub workspace
# if it is focused
# formats:
#   {index} - The sub index of the workspace
# default: "{index"
sub_display_name_focused = ""

# The characters to separate each name with
display_sep = " "

# The maximum amount of sub workspaces this preset can
# have. If left out there will be no maximum
# default: 
max_sub_count = 7

# If there is a maximum amount of sub workspaces, empty
# workspace names will print this name
# default: ""
display_name_empty = ""

# When creating this preset, the sub workspace that 
# should be focused
# default: 1
default_sub_workspace = 1

# When moving to an empty main workspace, whether to
# execute the given commands. Note: this only applies to
# the default preset
execute_on_move = false

# This will be removed
growable = true


[types.commands]

# A list of commands to execute on the corresponding sub
# workspaces
# example:
#    1 = ["kitty"]
#    2 = ["firefox"]Can be configured to print however you would like
#    3 = ["spotify", "steam"]
# default:
2 = ["steam"]
1 = ["kitty", "qutebrowser"]

# To create new presets, add a new [[types]] and 
# [types.commands] table to this file
```
<a name="todo"></a>
## TODO
- [x] Manage main workspaces
- [x] Manage sub workspaces
- [x] Create workspace preset
- [ ] Start applications when creating a preset
- [x] Move and swap main/sub workspaces
- [x] Make it configurable
- [x] Output something nice that things like polybar, eww, etc... can use
- [x] Create an install script
- [ ] Remove jq dependency
- [ ] Cleanup output/error messages
