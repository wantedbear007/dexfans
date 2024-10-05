#!/bin/bash

# Function to clean and run the project
clean_and_run() {
    echo "Cleaning up..."
    dfx stop
    dfx start --clean
    echo "Deploying canisters..."
    dfx deploy
    echo "Project started successfully after cleaning."
}

# Function to run without cleaning
run_without_cleaning() {
    echo "Running project without cleaning..."
    dfx stop
    dfx start
    echo "Deploying canisters..."
    dfx deploy
    echo "Project started successfully without cleaning."
}

# Script options
echo "Choose an option:"
echo "1) Clean and run the project"
echo "2) Run without cleaning"

# Read user input
read -p "Enter option (1 or 2): " option

case $option in
    1)
        clean_and_run
        ;;
    2)
        run_without_cleaning
        ;;
    *)
        echo "Invalid option. Please choose 1 or 2."
        ;;
esac