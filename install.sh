#!/bin/bash
#install using curl -sSL https://raw.githubusercontent.com/ji-soft/kubyplexer/main/install.sh | bash
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
echo "starting kubyplexer. Have fun :) "
cd kubyplexer
npm start
