#!/bin/bash

i3-msg -t get_tree | jq -r '.nodes[] | select(.name != "__i3") | .nodes[] | .nodes[] | select(.name == "'$1'") | .nodes[] | if .window != null then "true" else "" end'
