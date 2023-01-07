"use strict"

function load(canvas, module_path, options) {
    console.log("Loading", module_path)
    canvas.className = "loading"
    
    import(module_path)
    .then((module) => {
        module.default().then(function(obj){
            let core = new module.Core(canvas, options)
            core.start()
            canvas.core = core
        }).catch(function(e){
            console.error("Failed to init module:", e)
            canvas.className = "error"
        })
    }).catch(function(e) {
        console.error("Failed to load:", e)
        canvas.className = "error"
    });
}

function setup_canvas() {
    const canvases = document.querySelectorAll("canvas");
    for (let canvas of canvases) {
        let options = canvas.getAttribute("options") || ""
        let id = canvas.id.split("-")[0] // So we can have multiple canvas' with the same app and different options
        let module_path = '../gen/'+ id +'/game.js' // Path to WASM JS bindings
        canvas.tabIndex = 1
        canvas.addEventListener("click", function() {
            load(canvas, module_path, options)
        }, {'once':true})

        const linkContainer = document.createElement('div');
        linkContainer.style.display = 'flex'
        linkContainer.style.gap = '3em'

        const fullscreen = document.createElement('a')
        fullscreen.innerHTML = "Fullscreen"
        fullscreen.href = '../gen/'+ id +'/game.html'
        linkContainer.appendChild(fullscreen)

        const code = document.createElement('a')
        code.innerHTML = "Github"
        code.href = 'https://github.com/sdfgeoff/wasm_minigames/tree/master/src_rust'
        linkContainer.appendChild(code)


        canvas.parentElement.insertBefore(linkContainer, canvas.nextSibling)
    }
}
setup_canvas()
