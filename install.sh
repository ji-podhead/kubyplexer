#!/bin/bash
# Funktion zum Anzeigen blinkender Emojis
diamond_states=("🔹🔹🔹🔹🔹" "🔹🔹🔹🔹🔷" "🔹🔹🔹🔷🔷" "🔹🔹🔷🔷🔷" "🔹🔷🔷🔷🔷" "🔷🔷🔷🔷🔷")
diamond_states2=("🔹🔹🔹🔹🔹" "🔷🔹🔹🔹🔹" "🔷🔷🔹🔹🔹" "🔷🔷🔷🔹🔹" "🔷🔷🔷🔷🔹" "🔷🔷🔷🔷🔷" )
# Funktion zum Anzeigen blinkender Diamanten
show_diamonds() {
    # Schleife durch die Zustände der Diamanten
    for state in "${diamond_states[@]}"; do
        # Clear-Befehl, um die vorherige Zeile zu löschen
        clear
        # Diamanten ausgeben
        echo "$state"
        # Wartezeit einstellen, um die Geschwindigkeit der Animation zu kontrollieren
        sleep  0.5
    done

    # Richtungsänderung
    for ((i=${#diamond_states[@]}-1; i>=0; i--)); do
        clear
        echo "${diamond_states[$i]}"
        sleep  0.5
    done
}
fadein() {
    # Blinkmodus (0 = 🔹,   1 = 🔷)
    blink=0
    # Jogger-Zeichen
    jogger="🏃"
    jogger2="🏃"
    # Startposition des Joggers
    start_pos=$(tput cols)
    # Endposition des Joggers
    end_pos=0
    # Aktuelle Position des Joggers
    pos=$start_pos
    pos2=$start_pos
    # Blinkcounter für das Blinken der Emojis
    blink_counter=0
    local counter=0
    local max_states=${#diamond_states[@]}
    local end_pos=$((max_states *   2))
    local state_index=0;
    total_width=$(tput cols) # Gesamtbreite des Terminals
    text_width=$(echo -n "$state Kubyplexer $state2" | wc -c) # Breite des Textes
    padding=$(( (total_width - text_width) /   3   )) # Berechnung der Leerzeichen
    # Schleife für die Animation
    while [ $pos -ge $end_pos ]; do
            # Zustand basierend auf dem Zähler bestimmen
            # Position des Joggers aktualisieren
            pos=$((pos -   1))
            pos2=$((pos2 +   1))

            # Clear-Befehl, um die vorherige Zeile zu löschen
            clear
            echo ""
            echo ""

            # Zeichen an der aktuellen Position des Joggers ausgeben
            printf "%-*s" $pos ""
            echo ""
            echo ""
            echo ""

            # Blinkmodus umschalten
            if [ $((blink_counter %   8)) -eq   0 ]; then
                blink=0
            else
                blink=1
            fi
            # Emoji ausgeben, je nach Blinkmodus
            if [ $blink -eq   0 ]; then
                if (( $state_index <   5 )); then
                    ((state_index++))
                else
                    state_index=0
                fi
            fi    
            local state="${diamond_states[$state_index]}"
            local state2="${diamond_states2[$state_index]}"
            printf "%*s" $padding ""

            echo "$state >> Kubyplexer << $state2"



            # Diamanten hinter dem Jogger einsetzen
            for ((i=0; i<$pos; i++)); do
                printf "%-*s" $i ""
                printf "💎"
            done

            # Wartezeit einstellen, um die Geschwindigkeit der Animation zu kontrollieren
            sleep   0.02
            # Blinkcounter aktualisieren
            blink_counter=$((blink_counter +   1))
    done
}

show_blinking_emojis() {
    # Blinkmodus (0 = 🔹,   1 = 🔷)
    blink=0
    # Jogger-Zeichen
    jogger="🏃"
    jogger2="🏃"
    # Startposition des Joggers
    start_pos=$(tput cols)
    # Endposition des Joggers
    end_pos=0
    # Aktuelle Position des Joggers
    pos=$start_pos
    pos2=$start_pos
    # Blinkcounter für das Blinken der Emojis
    blink_counter=0
    local counter=0
    local max_states=${#diamond_states[@]}
    local end_pos=$((max_states *   2))
    local state_index=0;
    total_width=$(tput cols) # Gesamtbreite des Terminals
    text_width=$(echo -n "$state Kubyplexer $state2" | wc -c) # Breite des Textes
    padding=$(( (total_width - text_width) /   3   )) # Berechnung der Leerzeichen
    # Schleife für die Animation
    while [ $pos -ge $end_pos ]; do
            # Zustand basierend auf dem Zähler bestimmen
            # Position des Joggers aktualisieren
            pos=$((pos -   1))
            pos2=$((pos2 +   1))

            # Clear-Befehl, um die vorherige Zeile zu löschen
            clear
            echo ""
            echo ""

            # Zeichen an der aktuellen Position des Joggers ausgeben
            printf "%-*s" $pos ""
            echo "$jogger"
            echo ""
            echo ""

            # Blinkmodus umschalten
            if [ $((blink_counter %   8)) -eq   0 ]; then
                blink=0
            else
                blink=1
            fi
            # Emoji ausgeben, je nach Blinkmodus
            if [ $blink -eq   0 ]; then
                if (( $state_index <   5 )); then
                    ((state_index++))
                else
                    state_index=0
                fi
            fi    
            local state="${diamond_states[$state_index]}"
            local state2="${diamond_states2[$state_index]}"
            printf "%*s" $padding ""

            echo "$state >> Kubyplexer << $state2"

            echo ""
            echo ""
            printf "%-*s" $pos2 ""
            echo "$jogger2"

            # Wartezeit einstellen, um die Geschwindigkeit der Animation zu kontrollieren
            sleep   0.02
            # Blinkcounter aktualisieren
            blink_counter=$((blink_counter +   1))
    done
}


# Hauptfunktion, um die Animationen zu starten
main() {
    # Kubyplexer-Header
    echo "🎉 Kubyplexer 🎉"
    echo "_________________________________________________________________"

    # Animationen in Hintergrund-Prozessen starten
   # show_diamonds
   fadein
    show_blinking_emojis 

    # Wartezeit, um die Animationen anzuzeigen

    # Animationen stoppen (durch Senden von SIGINT)
    kill -INT $!
    kill -INT $!
    kill -INT $!

    # Cleanup
    clear
    echo "Animation beendet. Viel Spaß mit Kubyplexer!"
}

# Hauptfunktion aufrufen
main









#install using curl -SL https://raw.githubusercontent.com/ji-soft/kubyplexer/main/install.sh | bash
# Install wget
sudo apt-get update
sudo apt-get install -y wget
sudo dnf install wget
# Ask user if they want to install Python   3.9.18
# Ask user if they want to install Node.js
echo "_________________________________________________________________"
echo "                 ---->checking Nodejs <----"
echo "_________________________________________________________________"
read -p "Do you want to install Node.js? (y/n) " answer


if [ "$answer" != "${answer#[Yy]}" ] ;then
    # Install NVM
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash

    # Load NVM
    export NVM_DIR="$HOME/.nvm"
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
    [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion

    # Install Node.js
    nvm install node
else
    echo "Skipping Node.js installation."
fi
echo "_________________________________________________________________"
echo "                 ---->checking Python <----"
echo "if having dependency conflicts, pls install proper python version"
echo "_________________________________________________________________"
read -p "Do you want to install Python   3.9.18? (y/n) " answer
if [ "$answer" != "${answer#[Yy]}" ] ;then
    # Download Python   3.9.18
    wget https://www.python.org/ftp/python/3.9.18/Python-3.9.18.tgz
    tar -xzf Python-3.9.18.tgz
    cd Python-3.9.18

    # Configure and install Python   3.9.18
    ./configure --enable-optimizations
    make -j   2
    sudo make altinstall

    # Set Python   3.9 as default
    sudo update-alternatives --install /usr/bin/python python /usr/bin/python3.9   1

    # Install pip for Python   3.9
    curl https://bootstrap.pypa.io/get-pip.py -o get-pip.py
    python3.9 get-pip.py

    # Clean up
    cd ..
    rm -rf Python-3.9.18
    rm get-pip.py
else
    echo "Skipping Python   3.9.18 installation."
fi

#!/bin/bash

# Herunterladen des gesamten Repository als ZIP-Datei
wget https://github.com/ji-soft/kubyplexer/archive/refs/heads/main.zip -O kubyplexer.zip

# Erstellen des Zielordners, falls noch nicht vorhanden
mkdir -p kubyplexer

# Extrahieren des gewünschten Unterverzeichnisses in ein temporäres Verzeichnis
unzip kubyplexer.zip "kubyplexer-main/kubyplexer_js/*" -d temp_dir

# Verschieben des gewünschten Unterverzeichnisses in den Zielordner
mv temp_dir/kubyplexer-main/kubyplexer_js/* kubyplexer/

# Löschen des temporären Verzeichnisses und der ZIP-Datei
rm -rf temp_dir
rm kubyplexer.zip

echo "_________________________________________________________________"

echo "              ----> installing dependencies <----"
   mkdir -p ./kubyplexer/python_dependencies

    # Install required Python packages
    python3.9 -m pip install kubernetes --target=./kubyplexer/python_dependencies
echo "_________________________________________________________________"

echo "                ----> installation complete <----"
echo "run >>npm start<< to start the standalone version of kubyplexer"
echo "starting kubyplexer now. Have fun :) "
rm install.sh
cd kubyplexer
npm start
