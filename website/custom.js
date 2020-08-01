function load(id) {
    var module_path = './games/' + id + '/core.js'
    var canvas = document.getElementById(id)
    console.log("Loading", module_path)

    canvas.className="loading"
    canvas.onclick=null

    import(module_path)
    .then((module) => {
        module.default().then(function(obj){
            var core = new module.Core(id)
            core.start()
        }).catch(function(e){
            console.error("Failed to load:", e)
        })
    });

    
}
