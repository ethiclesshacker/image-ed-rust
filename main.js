import init, { get_dimensions_from_bytes, blur , get_dims} from './pkg/image_ed.js';
async function run() {
    console.log("New Script");
    await init();
    window.getHeight = get_dimensions_from_bytes;
    window.blur = blur;
    window.getDims = get_dims;
    console.log("WASM LOADED.");

}

run();