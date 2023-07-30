#!/bin/bash

function inst {
    sudo install target/release/shime /bin/shime
    ./res/colr i 0

    ./res/colr i 1
    sudo install target/release/shime /usr/bin/shime

    if ! (cat /etc/shells | grep shime > /dev/null); then
        ./res/colr i 2
        echo "/bin/shime" | sudo tee -a /etc/shells > /dev/null
        
        ./res/colr i 3
        echo "/usr/bin/shime" | sudo tee -a /etc/shells > /dev/null
    else
        printf ""
    fi
}

function build {
    ./res/colr b
    cargo build --release
}

if [[ $1 == "" ]]; then
    echo "what do you want to do?"
    echo "  - all - build and install"
    echo "  - build - build pkg"
    echo "  - install - install pkg in dir and read to /etc/shells
"
    printf "your choise: "
    read choise
    echo "$choise"

elif [[ $2 == "" ]]; then
    case $1 in
    "all" | "a")
        build
        unst
        ;;

    "install" | "i")
        inst
        ;;

    "build" | "b")
        build
        ;;
    
    *)
        echo $1 is not $0 command
        ;;
    esac
fi