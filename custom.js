function load(canvas, module_path) {
    console.log("Loading", module_path)
    canvas.className = "loading"
    
    import(module_path)
    .then((module) => {
        module.default().then(function(obj){
            var core = new module.Core(id)
            core.start()
        }).catch(function(e){
            console.error("Failed to init module:", e)
            canvas.className = "error"
        })
    }).catch(function(e) {
        console.error("Failed to load:", e)
        canvas.className = "error"
    });
}

const canvases = document.querySelectorAll("canvas");
for (canvas of canvases) {
    var id = canvas.id
    var module_path = './' + id + '/pkg/'+ id +'.js' // Path to WASM JS bindings
    canvas.tabIndex = 1
    canvas.addEventListener("click", function() {
        load(canvas, module_path)
    }, {'once':true})
}
