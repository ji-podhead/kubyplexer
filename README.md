# Kubyplexer 🚀
> a funky and Kubernetes-beginner-friendly CLI tool to manage your cluster using your mouse! 🎉
## Status 🚧
**Kubyplexer is currently in the prototype stage.** This means that while the core functionality is implemented, there may be bugs, incomplete features, and areas for improvement. We welcome feedback and contributions to help shape the future of Kubyplexer.

# about 📝
- manage your cluster `without the need of dashboards` directly in your terminal
- find possible methods, flags, and arguments without the need of API docs
- navigate using your mouse, shortcuts, and arrow-keys 🖱️
- start your dashboards with one click
- write your manifests directly in Kubyplexer
- sync other CLI tools (like k9s) that are running in different TTYs using the given selection
- free, free, and forever free! 💰

# ui prototype 🖥️
![preview](https://github.com/ji-soft/kubyplexer/blob/main/kubyplexer_noinfo.png?raw=true)

# motivation 💡
- Increase the Kubernetes workflow 🚀
- I struggled to find the name and port of my k.dashboard, so I wrote a little bash script to select and repeat kubectl commands by its index. *see my Kubehistory repo*
- So why don't we have a tool like htop but for K8s 🤔
- I think this could be a really cool project, and I believe it's a project where many people, especially Kubernetes-beginners, can benefit from 🌟

# planned features 🎯
- autogenerate commands from selection => edit and execute
- click on a history object in the kubehistory to repeat/edit
- double click a window for fullscreen
- use shift+arrow-key to cycle through the windows, release shift to scroll
- remove selection to get general commands, like get pods

# roadmap 🗺️
- find people to support me in making this 🤝
- decide whether to use python, an actual multiplexer, or to build in C 🔧
- do it! 💪

# connect 🌐
- You can find me via [Docker Discord Channel](https://discord.gg/HDnGNa68)
- I am "podhead", but in the React community you find me under "ji"
- DM is ok

# nodes 🧩
In Python, the `inspect` module can be used to find details and descriptions of classes and methods. It can even be employed to execute a function generically. The `getargspec` function is particularly useful for appending constructor arguments and flags. By adopting this general approach, you can avoid the extensive manual work required to create all the wrapper classes. While I haven't written in C or Java for quite some time😅  I'm confident that with the right guidance and resources, it's possible to accomplish this task using C or any other suitable language. 

# Implementation Plan 🛠️
- **Python Scripts**: Develop Python scripts to deliver the desired functionalities of the multiplexer.
- **Subprocess Library**: Utilize the `subprocess` module to execute commands and capture their output.
- **TTY Communication**: Handle TTY devices as files and use the `os` module to write to them. Consider using the `pty` module for managing pseudo-terminals.
- **Permissions**: Ensure your Python script has the necessary permissions to access TTY devices and execute commands.
- **Security**: Be mindful of security risks associated with command execution and data transmission to TTY devices.
- **Testing**: Rigorously test your script to confirm it operates as expected and to identify any unintended side effects.
