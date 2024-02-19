# Kubyplexer 🚀
> A funky and Kubernetes-beginner-friendly CLI tool to manage your cluster using your mouse! 🎉
## Status 🚧
**Kubyplexer is currently in the prototype stage.** This means that while the core functionality is implemented, there may be bugs, incomplete features, and areas for improvement. We welcome feedback and contributions to help shape the future of Kubyplexer.

# About 📝
- **Direct Cluster Management**: Manage your Kubernetes cluster directly in your terminal without the need for external dashboards.
- **Effortless Navigation**: Navigate using your mouse, shortcuts, and arrow keys to interact with your cluster. 🖱️
- **Immediate Dashboard Access**: Start your Kubernetes dashboards with a single click.
- **Manifest Creation**: Create and edit Kubernetes manifests directly within Kubyplexer.
- **Sync with CLI Tools**: Sync other CLI tools like k9s that are running in different TTYs using the given selection.
- **Free and Open Source**: Kubyplexer is free to use and will always be free! 💰

# UI Prototype 🖥️
![preview](https://github.com/ji-soft/kubyplexer/blob/main/kubyplexer_noinfo.png?raw=true)

# Motivation 💡
- **Enhance Kubernetes Workflow**: Kubyplexer aims to streamline the Kubernetes workflow by providing a user-friendly interface.
- **Addressing User Challenges**: Created in response to the challenge of finding the name and port of a Kubernetes dashboard, Kubyplexer simplifies the process of repeating and editing kubectl commands.
- **Inspired by Existing Tools**: Inspired by tools like htop, Kubyplexer aims to offer a similar utility for Kubernetes, making it easier for beginners to manage their clusters.

# Planned Features 🎯
- **Command Autogeneration**: Generate commands from selection for easy editing and execution.
- **Kubehistory Integration**: Click on a history object in the kubehistory to repeat or edit commands.
- **Fullscreen Mode**: Double-click a window to enter fullscreen mode.
- **Window Management**: Use shift+arrow-key to cycle through windows, release shift to scroll.
- **General Commands**: Remove selection to access general commands, like `get pods`.

# Roadmap 🗺️
- **Collaboration**: Find contributors to help develop Kubyplexer. 🤝
- **Technology Decision**: Decide on the technology stack, considering options like Python, a multiplexer, or building in C.
- **Development**: Begin the development process! 💪

# Connect 🌐
- **Docker Discord Channel**: Join me via the [Docker Discord Channel](https://discord.gg/HDnGNa68).
- **React Community**: I'm "podhead" on Discord and "ji" in the React community.
- **Direct Messages**: Feel free to send me a direct message.

# Implementation Plan 🛠️
- **Python Scripts**: Develop Python scripts to provide the multiplexer's functionalities.
- **Subprocess Library**: Use the `subprocess` module to execute commands and capture output.
- **TTY Communication**: Manage TTY devices with the `os` and `pty` modules.
- **Permissions**: Ensure the script has the necessary permissions to access TTY devices and execute commands.
- **Security**: Consider security risks associated with command execution and TTY device data transmission.
- **Testing**: Test the script rigorously to ensure it operates as expected and to identify any unintended side effects.

# Nodes 🧩
In Python, the `inspect` module can be used to inspect classes and methods, which is useful for generating wrapper classes and appending constructor arguments and flags. While htop was written in C, it's possible to create a similar tool in Python, potentially using a high-level language like C or Java for additional functionality such as mouse support. With the right guidance and resources, this task can be accomplished in under   3 months.
