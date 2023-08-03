#!/usr/bin/bash

BASE="$(dirname "$(readlink -f "$0")")"
PLUGIN="${BASE}/plugin"

function is_release() {
    if [ -d "${BASE}/target/release" ]; then
        return 0 # true
    else
        return 1 # false
    fi
}

function package() {
    if  is_release; then
        TARGET="${BASE}/target/release"
    else
        TARGET="${BASE}/target/debug"
    fi

    echo "Packaging at ${TARGET}/plugin"

    cp -r ${PLUGIN} ${TARGET}
    cp ${TARGET}/*.so ${TARGET}/plugin/contents/ui/WallpaperEngineKDE
    cp ${BASE}/metadata.desktop ${TARGET}/plugin
}

function install() {
    if  is_release; then
        TARGET="${BASE}/target/release"
    else
        TARGET="${BASE}/target/debug"
    fi

    echo "Installing from ${TARGET}/plugin"

    plasmapkg2 -i ${TARGET}/plugin 2>/dev/null || \
    plasmapkg2 -u ${TARGET}/plugin
}

function remove() {
    if  is_release; then
        TARGET="${BASE}/target/release"
    else
        TARGET="${BASE}/target/debug"
    fi

    echo "Removing from ${TARGET}/plugin"

    plasmapkg2 -r ${TARGET}/plugin
}

function help() {
    echo "Usage: install.sh [-i] [-r] [-p]"
    echo "  -i  Install"
    echo "  -r  Remove"
    echo "  -p  Package"
    echo "  -h  Help"
}

function main() {
    if [ $# -eq 0 ]; then
        help
        return 0
    fi  

    while getopts "irph" opt; do
        case ${opt} in
            i) 
                install
                ;;
            r) 
                remove
                ;;
            p) 
                package
                ;;
            h)
                help
                ;;
            \?)
                echo "Invalid option: -$OPTARG" 1>&2
                help
                exit 1
                ;;
        esac
    done  
}

main "$@"
