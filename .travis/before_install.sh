#!/bin/bash

if [[ $TRAVIS_OS_NAME == 'osx' ]]; then
	brew update
	brew install sdl2
	brew install sdl2_ttf
else
	sudo apt-get update -qq
	sudo apt-get install -y libsdl2-dev libsdl2-ttf-dev 
fi
