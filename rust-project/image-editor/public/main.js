function init() {
    let rustApp = null;


    try {
        rustApp = import('../pkg')
    } catch (err) {
        console.log(err);
        return
    }

    console.log(rustApp);
    

    const input = document.getElementById('upload');
    const fileReader = new FileReader();



    fileReader.onloadend = () => {
        let base64 = fileReader.result.replace(/^data:image\/(png|jpeg|jpg);base64,/, '');

        console.log(input.files[0]);

        console.log(base64);

    }


    input.addEventListener('change', () => {
        fileReader.readAsDataURL(input.files[0]);
    })
}

init()