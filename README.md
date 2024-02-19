# Kubyplexer
> a funky and beginnerfriendly cli tool to manage your cluster using your mouse! U+263A 	

<table>
  <thead>
    <tr>
      <th>planned features</th>
      <th>ui prototype</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>
        <table>
          <tbody>
            <tr>
              <td>manage your cluster without the need of exposing ports directly in your shell</td>
            </tr>
            <tr>
              <td>find possible methods,flags and arguments without the need of api docs </td>
            </tr>
            <tr>
              <td>navigate using your mouse, shortcuts and arrow-keys</td>
            </tr>
             <tr>
              <td>start your dashboards with one click</td>
            </tr>
            <tr>
              <td>write your manifests directly in Kubyplexer</td>
            </tr>
            <tr>
              <td>sync other cli tools (like k9s) that are running in different ttys using the given selection</td>
            </tr>
            <tr>
              <td>autogenerate commands from selection => edit and execute</td>
            </tr>
            <tr>
              <td>click on a history object in the kubehistory to repeat/edit</td>
            </tr>
            <tr>
              <td>double click a window for fullscreen</td>
            </tr>
            <tr>
              <td>use shift+arroy-key to cycle through the windows, release shift to scroll</td>
            </tr>
            <tr>
              <td>remove selection to get general commands, like get pods</td>
            </tr>
         </tbody>
        </table>
      </td>
      <td>
        <img src="https://github.com/ji-soft/kubyplexer/blob/main/kubyplexer_noinfo.png?raw=true" alt="preview" />
      </td>
    </tr>
  </tbody>
</table>

# motivation
- Increase the kubernetes workflow baby!
- I struggled to find the name and port of my k.dashboard, so i wrote a little bashscript to select and repeat kubectl commands by its index. *see my Kubehistory repo*  
- So why dont have tool like htop but for k8s 
- I think this could be a really cool project, and I believe it's a project where many people, especially beginners, can benefit from

# roadmap
- find people to support me making this 
- decide wheather to use an actual multiplexer, or to build in c
- do it!

# connect
- You can find me via [Docker Discord Channel](https://discord.gg/HDnGNa68)
- i am "podhead", but in the react community you find me under "ji"
- dm is ok

# nodes
In Python, you can use the inspect module to find details and descriptions of classes and methods. You can even use it to execute a function generically. The getargspec function can be used to append constructor arguments and flags. However, I don't think that my planned project can be written in Python, since a multiplexer with mouse support requires a higher-level programming language like C. This can be quite an advanced and labor-intensive project.

I haven't written in C or Java for quite some time, so I'm not sure if I can do this alone. 
