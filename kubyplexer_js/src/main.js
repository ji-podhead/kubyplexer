import * as slint from "slint-ui";
import { spawn } from 'child_process';

let ui = slint.loadFile("ui/appwindow.slint");
let window = new ui.AppWindow();
// Gemeinsame Datenstruktur
let sharedData = {
    data: {},
    lock: false
};
window.request_increase_value = function () {
    window.counter = window.counter +  1;
};
window.runPythonScript = function() {
    runPythonScript('src/kuby_py.py').then(data => {
        console.log("Data received from Python script:", data);
    }).catch(console.error);
};
function runPythonScript(scriptPath) {
    return new Promise((resolve, reject) => {
        const python = spawn('python', [scriptPath], { stdio: 'pipe' });
        let result = '';

        python.stdout.on('data', (data) => {
            result += data.toString();
        });

        python.stderr.on('data', (data) => {
            console.error(`Python script stderr: ${data}`);
        });

        python.on('close', (code) => {
            if (code !==  0) {
                return reject(new Error(`Python script exited with code ${code}`));
            }
            resolve(result);
        });
    });
}

// Beispielaufruf >> RELATIVE PATH << no ./ needed!!!
runPythonScript('src/kuby_py.py').then(data => {
    console.log("Data received from Python script:", data);
}).catch(console.error);

await window.run();