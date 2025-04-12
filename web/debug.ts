import chokidar from 'chokidar';
import { exec, ChildProcess } from 'child_process';

let buildProcess: ChildProcess | null = null;
let isBuilding = false;

const watcher = chokidar.watch('./src', { ignored: /^\./, persistent: true });

watcher.on('change', (path) => {
    console.log(`${path} has been changed`);

    if (isBuilding && buildProcess) {
        console.log('Killing the previous build process...');
        buildProcess.kill();
    }

    isBuilding = true;

    // Wait for a short period before starting the build
    setTimeout(() => {
        buildProcess = exec('bun run build', (error, stdout, stderr) => {
            isBuilding = false;

            if (error) {
                console.error(`exec error: ${error}`);
                return;
            }
            console.log(`stdout: ${stdout}`);
            console.error(`stderr: ${stderr}`);
        });
    }, 1000); // Wait for 1 second before starting the build
});