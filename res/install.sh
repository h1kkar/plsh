#!/bin/bash

function inst {
    sudo install target/release/shime /bin/shime
    ./res/colr 2

    ./res/colr 3
    sudo install target/release/shime /usr/bin/shime

    if ! (cat /etc/shells | grep shime > /dev/null); then
        ./res/colr 4
        echo "/bin/shime" | sudo tee -a /etc/shells > /dev/null
        
        ./res/colr 5
        echo "/usr/bin/shime" | sudo tee -a /etc/shells > /dev/null
    else
        printf ""
    fi
    ./res/colr 6
    mkdir $HOME/.config/shime > /dev/null
    ./res/colr 7
    touch $HOME/.config/shime/shime_history > /dev/null
}

function build {
    ./res/colr 1
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