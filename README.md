# Kubyplexer 🚀
> a funky and Kubernetes-beginner-friendly CLI tool to manage your cluster using your mouse! 🎉

# about 📝
- manage your cluster without the need of dashboards directly in your terminal
- find possible methods, flags, and arguments without the need of API docs
- navigate using your mouse, shortcuts, and arrow-keys
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
- I think this could be a really cool project, and I believe it's a project where many people, especially beginners, can benefit from 🌟

# planned features 🎯
- autogenerate commands from selection => edit and execute
- click on a history object in the kubehistory to repeat/edit
- double click a window for fullscreen
- use shift+arrow-key to cycle through the windows, release shift to scroll
- remove selection to get general commands, like get pods

# roadmap 🗺️
- find people to support me in making this 🤝
- decide whether to use an actual multiplexer, or to build in C 🔧
- do it! 💪

# connect 🌐
- You can find me via [Docker Discord Channel](https://discord.gg/HDnGNa68)
- I am "podhead", but in the React community you find me under "ji"
- DM is ok

# nodes 🧩
In Python, you can use the `inspect` module to find details and descriptions of classes and methods. You can even use it to execute a function generically. The `getargspec` function can be used to append constructor arguments and flags. However, I don't think that my planned project can be written in Python, since a multiplexer with mouse support requires a higher-level programming language like C. This can be quite an advanced and labor-intensive project.

I haven't written in C or Java for quite some time, so I'm not sure if I can do this alone. 😅
