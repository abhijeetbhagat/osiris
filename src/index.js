const { ipcRenderer } = require('electron')
let parser = require("parser")


//Dragdrop event listener
var dragFile = document.getElementById("drag-file");
dragFile.addEventListener('drop', function (e) {
    e.preventDefault();
    e.stopPropagation();

    for (let f of e.dataTransfer.files) {
        console.log('The file(s) you dragged: ', f)
        document.write(`<h1>${JSON.stringify(parser.parse(f.path))}</h1>`);
    }
});

dragFile.addEventListener('dragover', function (e) {
    e.preventDefault();
    e.stopPropagation();
});




