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

  $('.header').hide();
  $('#window').jstree({
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

window.addEventListener("DOMContentLoaded", () => {
  const menuButton = document.getElementById("menu-btn");
  const minimizeButton = document.getElementById("minimize-btn");
  const maxUnmaxButton = document.getElementById("max-unmax-btn");
  const closeButton = document.getElementById("close-btn");

  menuButton.addEventListener("click", e => {
    // Opens menu at (x,y) coordinates of mouse click on the hamburger icon.
    window.openMenu(e.x, e.y);
  });

  minimizeButton.addEventListener("click", e => {
    window.minimizeWindow();
  });

  maxUnmaxButton.addEventListener("click", e => {
    const icon = maxUnmaxButton.querySelector("i.far");

    window.maxUnmaxWindow();

    // Change the middle maximize-unmaximize icons.
    if (window.isWindowMaximized()) {
      icon.classList.remove("fa-square");
      icon.classList.add("fa-clone");
    } else {
      icon.classList.add("fa-square");
      icon.classList.remove("fa-clone");
    }
  });

  closeButton.addEventListener("click", e => {
    window.closeWindow();
  });
});

