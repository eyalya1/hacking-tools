#! /bin/bash
safe=""
if [ $# -eq 0 ]; then
    filename="internationPhoneNumberGen.cpp"
else
    for arg in "$@"; do
        if [ $arg == "safe" ]; then
            safe="--random-mac"
        else
            filename=$arg
        fi
    done
    filename=$1
fi
if [ -f "PhoneNumbers.txt" ]; then
    sudo wifite --daemon $safe --dict PhoneNumbers.txt
else
    if [ -f "interPhoneNumbers.txt" ]; then
        sudo wifite --daemon $safe --dict interPhoneNumbers.txt
    else
        g++ -o internationPhoneNumberGen "$filename"
        ./internationPhoneNumberGen
    fi
fi