required_arg() {
    if [ -z "$1" ]; then
        echo "Required argument $2 missing or empty"
        exit 1
    fi
}