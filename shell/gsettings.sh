#!/bin/bash

gsettings set org.gnome.desktop.background picture-uri file://$1
exit $?