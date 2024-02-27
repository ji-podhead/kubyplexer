<p align="center">
  <img src="https://github.com/ji-soft/kubyplexer/blob/main/images/kubyplexer_small2.jpg?raw=true" alt="logo" />
</p>

<p align="center" style="font-size:  1.2em;">
  <strong>A funky and Kubernetes-beginner-friendly tool to manage your cluster using your mouse! 🖱️  </strong>
  </p>
  <p align="center" style="font-size:  1.0em;"> 🚀 Kubyplexer is becoming a VS code extension 🚀 :) :)
  </p>

# About 📝
- **Direct Cluster Management**: Manage your Kubernetes cluster directly in your vscode/app window without the need for external dashboards.
- **Effortless Navigation**: Navigate using your mouse, shortcuts, and arrow keys to interact with your cluster. 
- **Dashboard Access**: Start your Kubernetes dashboards with a single click and use them inside kubiplexer. 
- **Manifest Creation**: Create and edit Kubernetes manifests directly within Kubyplexer.
- **Sync with CLI Tools**: Sync other CLI tools like k9s that are running in different TTYs using the given selection.
- **Free and Open Source**: Kubyplexer is free to use and will always be free! 💰

# Planned Features 🎯
- **Command Autogeneration**: Generate commands from selection for easy editing and execution.
- **Kubehistory Integration**: Click on a history object in the kubehistory to repeat or edit commands.
- **Fullscreen Mode**: Double-click a window to enter fullscreen mode.
- **Window Management**: Use shift+arrow-key to cycle through windows, release shift to scroll.
- **General Commands**: Remove selection to access general commands, like `get pods`.

# UI Prototype 🖥️
<p align="center">
  <img src="https://github.com/ji-soft/kubyplexer/blob/main/images/kubyplexer_noinfo.png?raw=true" />
</p>

# Motivation 💡
- **Enhance Kubernetes Workflow**: Kubyplexer aims to streamline the Kubernetes workflow by providing a user-friendly interface.
- **Addressing User Challenges**: Created in response to the challenge of finding the name and port of a Kubernetes dashboard, Kubyplexer simplifies the process of repeating and editing kubectl commands.
- **Inspired by Existing Tools**: Inspired by tools like htop, Kubyplexer aims to offer a similar utility for Kubernetes, making it easier for beginners to manage their clusters.

# Status 🚧
**Kubyplexer is currently in the prototype stage.** This means that while the core functionality is implemented, there may be bugs, incomplete features, and areas for improvement. We welcome feedback and contributions to help shape the future of Kubyplexer.

# Install 💾
- run  `curl https://raw.githubusercontent.com/ji-soft/kubyplexer/main/install.sh > install.sh && bash install.sh` 
- install script will create some folders, installing dependencies, clean up and start the standalone version
- for manuall install, pls create folder named python_dependencies inside kubyplexer and install dependencies there
  
# Roadmap 🗺️
- **Collaboration**: Find contributors to help develop Kubyplexer. 🤝
- **Technology Decision**: Decide on the technology stack, considering options like Python, a multiplexer, or building in C.
- **Development**: Begin the development process! 💪
- **Community Engagement**: Create a website for Kubyplexer to introduce it to the community and gather feedback on desired features and improvements.
- **Feature Prioritization**: Engage with the community to determine which features are most needed and which can be deferred or omitted.

# Connect 🌐
- ~~**Docker Discord Channel**: Join me via the [Docker Discord Channel](https://discord.gg/HDnGNa68).~~
- **Twitter/X**: Find me via "ji.podhead"
- **React Community**: In the React Community my name is "ji".
- **Direct Messages**: Feel free to send me a direct message if you have any questions, or if you want to contribute.
  
# Implementation Plan 🛠️
- use slint as gui.tview wont get me further. But nodejs and python seem to be perfect solution.
- ~~create a hook-funktionality for cluster properties => use notify crate in rust~~
- ~~Golang Scripts: Develop Golang scripts to provide the multiplexer's functionalities using [tview](https://github.com/rivo/tview)~~  
- Python Scripts: Develop Python scripts to provide the multiplexer's functionalities.
- Subprocess Library: Use the `subprocess` module to execute commands and capture output.
- ~~TTY Communication: Manage TTY devices with the `os` and `pty` modules.~~
- Permissions: Ensure the script has the necessary permissions to access TTY devices and execute commands.
- Security: Consider security risks associated with command execution and TTY device data transmission.
- testing: Test the script rigorously to ensure it operates as expected and to identify any unintended side effects.
- ~~Mouse Support: Implement mouse support using libraries such as `curses` to handle mouse events within the terminal.~~


# Nodes 🧩
> In Python, the inspect module is particularly useful for inspecting classes and methods, which can aid in generating wrapper classes and appending constructor arguments and flags. Although tools like htop are written in C, it is feasible to create a similar utility in Python, potentially using a high-level language like C or Java to simplify the implementation of features such as mouse support. With the appropriate guidance, support, and resources, I am confident that the core functionality of Kubyplexer can be developed within a timeframe of less than three months!" 💪

> Docker Discord was disappointing. I asked a simnple question and they told me to hire a consultant. 
> ~~I decided to choose golang as the main language. but maybe i use python to implement the generic stuff~~
>tview cant provide scrollbars, nor it gives you the possibility to create your own, so now its on slint to get it done
>slint can also render to console, however there are some tasks to be solved > [see this discussion](https://github.com/slint-ui/slint/discussions/4672) 
>rust is not very friendly when it comes to generic stuff and it sooo much redundant boilerplate code that I decided to use nodejs + python +slint :)
>with nodejs we can create a vscode extrension in no time so **kubyplexer is becoming a vs code extension** 
