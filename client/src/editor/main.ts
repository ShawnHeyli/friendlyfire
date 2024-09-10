import { fabric } from 'fabric';

const canvas = new fabric.Canvas("preview");

function renderTemplate() {
    canvas.clear();
    canvas.setDimensions({ width: 1020, height: 1446});
    canvas.setBackgroundImage("/src/editor/preview.jpg", canvas.renderAll.bind(canvas));
}

renderTemplate()
