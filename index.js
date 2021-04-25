let image = document.querySelector("img");
const fileinput = document.querySelector("#file-input");

function fileUpdate() {

    var file = fileinput.files[0];
    var reader = new FileReader();
    reader.onload = async function () {
        byteArray = new Uint8Array(reader.result);
        console.log(byteArray.byteLength);

        // Perform operations on byteArray
        let newByteArray = byteArray;
        console.log("Height: ", getDims(newByteArray));
        console.log("Old");
        console.log(getHeight(newByteArray));
        console.log(byteArray);
        // console.log("Height: ", getHeight(newByteArray));

        /**
        console.log("Starting blur...");
        newByteArray = new Uint8Array(blur(newByteArray, 4.0));

        image.src = URL.createObjectURL(
            new Blob(
                [newByteArray.buffer],
                { type: 'image/png' }
            )
        );
        */

    }
    if (file) {
        reader.readAsArrayBuffer(file);
    }
}
