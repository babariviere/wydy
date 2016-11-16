#!/bin/bash

if [[ $TRAVIS_OS_NAME == 'osx' ]]; then
	brew update
	brew install sdl2
else
	sudo apt-get update -qq
	sudo apt-get install -y libsdl2 libsdl2-dev libsdl2-ttf libsdl2-ttf-dev 
fi
