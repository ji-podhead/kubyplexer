<p align="center">
  <img src="https://github.com/ji-soft/kubyplexer/blob/main/images/kubyplexer_small2.jpg?raw=true" alt="logo" />
</p>

<p align="center" style="font-size:  1.2em;">
  <strong>A funky and Kubernetes-beginner-friendly CLI tool to manage your cluster using your mouse! 🚀</strong>
</p>

# About 📝
- **Direct Cluster Management**: Manage your Kubernetes cluster directly in your terminal without the need for external dashboards.
- **Effortless Navigation**: Navigate using your mouse, shortcuts, and arrow keys to interact with your cluster. 🖱️
- **Immediate Dashboard Access**: Start your Kubernetes dashboards with a single click.
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

# Roadmap 🗺️
- **Collaboration**: Find contributors to help develop Kubyplexer. 🤝
- **Technology Decision**: Decide on the technology stack, considering options like Python, a multiplexer, or building in C.
- **Development**: Begin the development process! 💪
- **Community Engagement**: Create a website for Kubyplexer to introduce it to the community and gather feedback on desired features and improvements.
- **Feature Prioritization**: Engage with the community to determine which features are most needed and which can be deferred or omitted.

# Connect 🌐
- **Docker Discord Channel**: Join me via the [Docker Discord Channel](https://discord.gg/HDnGNa68).  I'm "podhead".
- **React Community**: In the React Community my name is "ji".
- **Direct Messages**: Feel free to send me a direct message if you have any questions, or if you want to contribute.
  
# Implementation Plan 🛠️
- **Python Scripts**: Develop Python scripts to provide the multiplexer's functionalities.
- **Subprocess Library**: Use the `subprocess` module to execute commands and capture output.
- **TTY Communication**: Manage TTY devices with the `os` and `pty` modules.
- **Permissions**: Ensure the script has the necessary permissions to access TTY devices and execute commands.
- **Security**: Consider security risks associated with command execution and TTY device data transmission.
- **Testing**: Test the script rigorously to ensure it operates as expected and to identify any unintended side effects.
- **Mouse Support**: Implement mouse support using libraries such as `curses` to handle mouse events within the terminal.

# Nodes 🧩
In Python, the `inspect` module can be used to inspect classes and methods, which is useful for generating wrapper classes and appending constructor arguments and flags. While htop is written in C, it is possible to create a similar tool in Python and perhaps use a high-level language such as C or Java for a simpler approach to features such as mouse support. I hope that the right guidance, support and resources, this core functionality can be completed in less than 3 months! 💪
