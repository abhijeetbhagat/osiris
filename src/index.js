const { ipcRenderer } = require('electron')

//Dragdrop event listener
var dragFile = document.getElementById("drag-file");
dragFile.addEventListener('drop', function (e) {
    e.preventDefault();
    e.stopPropagation();

    for (let f of e.dataTransfer.files) {
        console.log('The file(s) you dragged: ', f)
    }
});

dragFile.addEventListener('dragover', function (e) {
    e.preventDefault();
    e.stopPropagation();
});




