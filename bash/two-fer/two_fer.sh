#!/usr/bin/env bash

resistor_color_duo() {
    # declare an hash map
    declare -A colours
    colours=(
        ["black"]=0
        ["brown"]=1
        ["red"]=2
        ["orange"]=3
        ["yellow"]=4
        ["green"]=5
        ["blue"]=6
        ["violet"]=7
        ["grey"]=8
        ["white"]=9
        )

    # check for invalid colour
    if [[ ! ${colours["$1"]} || ! ${colours["$2"]} ]]; then
        echo "invalid color" && exit 1;
    fi

    echo "${colours["$1"]}${colours["$2"]}"
}

resistor_color_duo "$@"
