#!/bin/bash

avrdude -c usbtiny -p attiny85 -U flash:w:$1