#!/bin/bash

# Install sixtyfps-viewer (do nothing if it is already installed)
cargo install sixtyfps-viewer

output=$(sixtyfps-viewer - --save-data - << EOF
import { StandardButton, LineEdit, GridBox } from "sixtyfps_widgets.60";
_ := Dialog {
    StandardButton { kind: ok; }
    StandardButton { kind: cancel; }
    property name <=> name-le.text;
    property address <=> address-le.text;
    GridBox {
        Row {
            Text { text: "Enter your name:"; }
            name-le := LineEdit { }
        }
        Row {
            Text { text: "Address:"; }
            address-le := LineEdit { }
        }
    }
}
EOF
)
if [ $? -eq  0 ]; then
    name=`echo $output | grep -o '"name": *"[^"]*"' | grep -o '"[^"]*"$'`
    address=`echo $output | grep -o '"address": *"[^"]*"' | grep -o '"[^"]*"$'`
    echo "Your name is $name and you live in $address"
fi
