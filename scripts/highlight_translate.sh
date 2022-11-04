#!/bin/bash

regex="[\p{Cyrillic}]"
input=$( xclip -o )
is_ru=$( echo "$input" | rg -e "$regex")
if [[ $is_ru ]]; then
    lang="ru-en"
else
    lang="en-ru"
fi

ydictionary lookup $lang "$input" |& less -~