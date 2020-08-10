function load(id) {
    var module_path = '/' + id + '/core.js'
    var canvas = document.getElementById(id)
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
    canvas.tabIndex = 1
    canvas.addEventListener("click", function() {
        load(canvas.id)
    }, {'once':true})
}
