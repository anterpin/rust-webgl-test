const rust = import('./engine');
[]

window.parseImage = async(src) => {
  return await new Promise((resolve, reject) => {
    let img = new Image()
    img.onload = () => resolve(img)
    img.onerror = reject
    img.src = src
  })
}

rust.then(async(func)=>{
    await func.init();
    const callback = ()=>{
        func.prepare();

        func.draw();
        requestAnimationFrame(callback);
    }
    requestAnimationFrame(callback);
    // func.set_value(30);
    // func.start();
    // func.get_value();
    // func.greet("Antonio");
}).catch(console.error);