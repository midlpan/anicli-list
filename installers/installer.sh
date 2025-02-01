#!/usr/bin/bash

printf "You are running an installer for the anicli-list program. A program that serves to list animes in the terminal.\n"
printf "\n"
printf "dependencies: [rust/cargo]\n"

read -p "Do you want to continue with the installation[Y|N]?:" question

if [ $question == "N" ];
then
        printf "Installer stopped\n"
        exit 0
elif [ $question == "Y" ];
then

        read -p "In which directory did you install the repository?:" installation_dir
        if [ ! -d "$installation_dir" ]; then
                echo "$installation_dir does not exist."
                exit 1
        fi

        cd $installation_dir
        cargo build -r
        sudo cp target/release/anicli-list /usr/bin
        mkdir ${HOME}/.config/anicli-list
        printf "It is highly recommended that you have a separate directory to store anime lists\n"
        printf "example: \n"
        printf "        /home/user_penguin/Documents/animes/db/\n"
        printf "\n"
        read -p "What is the directory in which you want the animes to be saved?:" anime_directory
        if [ ! -d "$anime_directory" ]; then
                echo "$anime_directory does not exist."
                exit 1
        fi
        echo "$anime_directory" > ${HOME}/.config/anicli-list/path.conf
        printf "anicli-list installation completed successfully\n"
else
        echo "The chosen option does not exist."
fi
