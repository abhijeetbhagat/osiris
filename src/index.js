const { ipcRenderer } = require('electron')
let parser = require("parser")
let $ = require('jquery');
let jstree = require("jstree");


//Dragdrop event listener
var dragFile = document.getElementById("drag-file");
dragFile.addEventListener('drop', function (e) {
    e.preventDefault();
    e.stopPropagation();

    let file = e.dataTransfer.files[0];

    console.log('The file(s) you dragged: ', file);
    //document.write(`<h1>${JSON.stringify(parser.parse(f.path))}</h1>`);
    let tree = parser.parse(file.path);
    console.log('parsing result: ', tree);
    let result = [];
    transform(tree.atoms, result);

    $('#container').jstree({
        'core': {
            data: result
        }
    });
});

dragFile.addEventListener('dragover', function (e) {
    e.preventDefault();
    e.stopPropagation();
});

function transform(atoms, result) {
    atoms.forEach(atom => {
        result.push({ 'text': atom.name });

        if (atom.atoms != undefined) {
            result[result.length - 1]['children'] = [];
            transform(atom.atoms, result[result.length - 1]['children']);
        }
    });
}