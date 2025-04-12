#!/bin/bash

validate_phone_numbers() {
    grep -E '^((\([0-9]{3}\) )|([0-9]{3}-))[0-9]{3}-[0-9]{4}$' file.txt
}

validate_phone_numbers